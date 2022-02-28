#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Parent {

    #[proxy]
    fn funder_proxy(&self) -> funder::Proxy<Self::Api>;
    
    #[init]
    fn init(&self,
        funder_address: ManagedAddress,
    ) {
        self.sc_funder_address().set_if_empty(&funder_address);
        self.is_registered().set_if_empty(&false);
    }


    #[payable("EGLD")]
    #[only_owner]
    #[endpoint(register)]
    fn register(
        &self,
        #[payment] payment: BigUint,
    ) {
        let to = self.sc_funder_address().get();
        self.funder_proxy()
            .contract(to)
            .accept_funds(TokenIdentifier::egld(), 0, payment)
            .async_call()
            .call_and_exit();
    }

    #[callback]
    fn register_callback(&self, #[call_result] result: ManagedAsyncCallResult<bool>) {
        match result {
            ManagedAsyncCallResult::Ok(value) => {
                self.is_registered().set(&value);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.is_registered().set(&false);
            }
        }
    }

    #[storage_mapper("scFunderAddress")]
    fn sc_funder_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("isRegistered")]
    fn is_registered(&self) -> SingleValueMapper<bool>;
}