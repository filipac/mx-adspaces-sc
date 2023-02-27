multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait Events {
    #[event("spaceBought")]
    fn space_bought(
        &self,
        #[indexed] name: &ManagedBuffer<Self::Api>,
        #[indexed] buyer: &ManagedAddress<Self::Api>,
        #[indexed] paid: &BigUint<Self::Api>,
        #[indexed] paid_until: &BigUint<Self::Api>,
    );

    #[event("testEvent")]
    fn test_event(&self, #[indexed] name: &ManagedBuffer<Self::Api>);
}
