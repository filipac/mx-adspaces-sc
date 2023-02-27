pub mod public {
    multiversx_sc::imports!();
    use crate::config::MINIMUM_PRICE;
    use crate::structs::AdvertiseSpace;
    use alloc::format;

    // const THIRTY_DAYS: u64 = 30 * 24 * 60 * 60;
    const TEN_DAYS: u64 = 10 * 24 * 60 * 60;

    #[multiversx_sc::module]
    pub trait PublicEndpoints:
        crate::storage::Storage + crate::events::Events + crate::views::Views
    {
        #[endpoint(buySpace)]
        #[payable("*")]
        fn buy_space(&self, space_name: &ManagedBuffer<Self::Api>) -> SCResult<&str> {
            let required_payment;
            let space_storage = self.spaces(space_name);
            let mut space: AdvertiseSpace<Self::Api> = self.get_space(space_name);
            if space.is_new {
                required_payment = space.paid_amount;
            } else {
                required_payment = space.paid_amount.mul(BigUint::from(2u32));
            }

            let caller = self.blockchain().get_caller();

            let mut transfer: Option<EsdtTokenPayment> = None;

            if caller != self.blockchain().get_owner_address() {
                transfer = Some(self.call_value().single_esdt());

                require!(
                    transfer.clone().unwrap().token_nonce == 0,
                    "Sent token nonce must be 0"
                );

                let t = transfer.clone().unwrap();

                // let token_id = t.token_identifier.as_managed_buffer();
                // let token_id = token_id.to_boxed_bytes();
                // let token_id = token_id.as_slice();

                // panic!("Test mode on.");

                require!(
                    self.known_tokens().contains(&t.token_identifier),
                    "Token not allowed. Only USDT and USDC are allowed."
                );

                // require!(
                //     ALLOWED_TYPES.contains(&token_id),
                //     "Token not allowed. Only USDT and USDC are allowed."
                // );
            }

            require!(space.owner != caller, "You already own this space");

            let min = BigUint::from(MINIMUM_PRICE);

            let paid = match transfer {
                Some(t) => t.amount,
                None => min / 2u64, // 3500000 usdc, 3,5 usdt/c
            };

            if caller != self.blockchain().get_owner_address() {
                let readable = &required_payment;
                let formatted = format!(
                    "Required payment is _{}_ (/10^6) USDT/USDC",
                    readable.to_u64().unwrap()
                );
                let err = formatted.as_str();

                require!(paid >= required_payment, err);
            }

            space.owner = caller;
            space.paid_amount = paid.clone();
            space.paid_until =
                BigUint::from(self.blockchain().get_block_timestamp()).add(BigUint::from(TEN_DAYS));
            space.is_new = false;
            space_storage.set(&space);

            self.space_bought(
                space_name,
                &space.owner,
                &space.paid_amount,
                &space.paid_until,
            );

            self.test_event(&ManagedBuffer::from("Space bought successfully"));

            return SCResult::Ok("Space bought successfully");
        }
    }
}

pub mod admin {
    use alloc::borrow::ToOwned;

    multiversx_sc::imports!();

    #[multiversx_sc::module]
    pub trait AdminEndpoints: crate::storage::Storage {
        #[endpoint(withdraw)]
        #[only_owner]
        fn withdraw(&self) -> SCResult<()> {
            let mut payments = ManagedVec::new();

            let accepted_tokens = self.accepted_tokens().get();

            // loop through all ALLOWED_TYPES and withdraw all tokens
            for token in accepted_tokens.iter() {
                let token_nonce = 0u64;

                let token = token.as_managed_buffer();

                let token_id = EgldOrEsdtTokenIdentifier::esdt(token.to_owned());
                let amount = self.blockchain().get_sc_balance(&token_id, token_nonce);

                if amount > 0 {
                    let payment = EsdtTokenPayment::new(
                        TokenIdentifier::from(token_id.into_name()),
                        0,
                        amount,
                    );

                    payments.push(payment);
                }
            }

            require!(
                payments.len() > 0,
                "No tokens to withdraw. Please try again later."
            );

            self.send()
                .direct_multi(&self.blockchain().get_owner_address(), &payments);
            Ok(())
        }

        #[endpoint(resetSpace)]
        #[only_owner]
        fn reset_space(&self, space_name: &ManagedBuffer<Self::Api>) -> SCResult<&str> {
            let space = self.spaces(space_name);

            require!(!space.is_empty(), "Space does not exist");

            space.clear();

            return SCResult::Ok("Space cleared");
        }

        #[only_owner]
        #[endpoint(addAcceptedTokens)]
        fn add_accepted_tokens(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
            let mut all_tokens_vec = self.accepted_tokens().get();
            let known_tokens_mapper = self.known_tokens();
            for token in tokens {
                require!(token.is_valid_esdt_identifier(), "Invalid token ID");

                if !known_tokens_mapper.contains(&token) {
                    known_tokens_mapper.add(&token);
                    all_tokens_vec.push(token);
                }
            }

            self.accepted_tokens().set(&all_tokens_vec);
        }

        #[only_owner]
        #[endpoint(removeAcceptedTokens)]
        fn remove_accepted_tokens(&self, tokens: MultiValueEncoded<TokenIdentifier>) {
            let mut all_tokens_vec = self.accepted_tokens().get();
            let known_tokens_mapper = self.known_tokens();
            for token in tokens {
                if known_tokens_mapper.contains(&token) {
                    known_tokens_mapper.remove(&token);

                    unsafe {
                        let index = all_tokens_vec.find(&token).unwrap_unchecked();
                        all_tokens_vec.remove(index);
                    }
                }
            }

            self.accepted_tokens().set(&all_tokens_vec);
        }
    }
}
