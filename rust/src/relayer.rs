use std::sync::Arc;

use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest, U256},
};

use eyre::Result;

pub struct Relayer {
    wallet: LocalWallet,
    provider: Provider<Ws>,
}

impl Relayer {
    pub fn new(wallet: LocalWallet, provider: Provider<Ws>) -> Self {
        Relayer { wallet, provider }
    }

    pub fn get_address(&self) -> Address {
        self.wallet.address()
    }

    pub async fn send_transaction(&self, to: Address, value: U256) -> Result<()> {
        let address = self.wallet.address().clone();
        let wallet = self.wallet.clone();

        let provider = self.provider.clone();

        let provider = Arc::new(provider.nonce_manager(address)).with_signer(wallet);

        let tx = TransactionRequest::new()
            .to(to)
            .value(value)
            .from(self.wallet.address());

        let tx_result = provider.send_transaction(tx, None).await;

        match tx_result {
            Ok(_) => Ok(()),
            Err(e) => panic!("Error sending tx: {}", e),
        }
    }
}
