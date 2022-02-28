#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Funder {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(acceptFunds)]
    fn accept_funds(
        &self,
        #[payment_token] _token: TokenIdentifier,
        #[payment_nonce] _nonce: u64,
        #[payment_amount] _payment: BigUint,
    ) -> bool {
        true
    }
}