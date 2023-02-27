#![no_std]

extern crate alloc;

multiversx_sc::imports!();

mod config;
mod events;
mod methods;
mod storage;
mod structs;
mod views;

#[multiversx_sc::contract]
pub trait ContractBlog:
    crate::storage::Storage
    + crate::events::Events
    + crate::views::Views
    + crate::methods::public::PublicEndpoints
    + crate::methods::admin::AdminEndpoints
{
    #[init]
    fn init(&self) {}
}
