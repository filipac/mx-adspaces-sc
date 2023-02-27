multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::config::MINIMUM_PRICE;
use multiversx_sc::types::BigUint;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct AdvertiseSpace<M: ManagedTypeApi> {
    pub owner: ManagedAddress<M>,
    pub paid_amount: BigUint<M>,
    pub paid_until: BigUint<M>,
    pub is_new: bool,
}

impl<M: ManagedTypeApi> AdvertiseSpace<M> {
    pub(crate) fn new_space(owner: ManagedAddress<M>) -> Self {
        AdvertiseSpace {
            owner: owner,
            paid_amount: BigUint::from(MINIMUM_PRICE),
            paid_until: BigUint::zero(),
            is_new: true,
        }
    }
}
