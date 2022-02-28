#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Test {
    #[proxy]
    fn funder_proxy(&self) -> funder::Proxy<Self::Api>;
    
    #[init]
    fn init(&self,
        funder_address: ManagedAddress,
    ) {
        self.sc_funder_address().set_if_empty(&funder_address);
    }

    #[payable("*")]
    #[only_owner]
    #[endpoint(register)]
    fn register(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] payment_nonce: u64,
        #[payment_amount] payment_amount: BigUint
    ) {
        let address = self.sc_funder_address().get();
        self.funder_proxy()
            .contract(address)
            .accept_funds(token, payment_nonce, payment_amount)
            .async_call()
            .call_and_exit();
    }

    #[storage_mapper("scFunderAddress")]
    fn sc_funder_address(&self) -> SingleValueMapper<ManagedAddress>;
}
