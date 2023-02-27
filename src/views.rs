multiversx_sc::imports!();
use crate::structs::AdvertiseSpace;
#[multiversx_sc::module]
pub trait Views: crate::storage::Storage {
    #[view(getSpace)]
    fn get_space(&self, space_name: &ManagedBuffer<Self::Api>) -> AdvertiseSpace<Self::Api> {
        let space = self.spaces(space_name);
        if !space.is_empty() {
            let current: AdvertiseSpace<Self::Api> = space.get();
            if current.paid_until > self.blockchain().get_block_timestamp() {
                return current;
            }
        }

        AdvertiseSpace::new_space(self.blockchain().get_sc_address())
    }

    #[view(getAcceptedTokens)]
    fn get_all_tokens(&self) -> MultiValueEncoded<TokenIdentifier> {
        self.accepted_tokens().get().into()
    }
}
