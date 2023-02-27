multiversx_sc::imports!();
use crate::structs::AdvertiseSpace;

#[multiversx_sc::module]
pub trait Storage {
    #[storage_mapper("spaces")]
    fn spaces(
        &self,
        space_name: &ManagedBuffer<Self::Api>,
    ) -> SingleValueMapper<AdvertiseSpace<Self::Api>>;

    // #[view(getAcceptedLockedAssetsTokenIds)]
    #[storage_mapper("acceptedTokens")]
    fn accepted_tokens(&self) -> SingleValueMapper<ManagedVec<TokenIdentifier>>;

    #[storage_mapper("knownTokens")]
    fn known_tokens(&self) -> WhitelistMapper<Self::Api, TokenIdentifier>;
}
