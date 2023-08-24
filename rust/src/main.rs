use std::{env, str::FromStr, sync::Arc};

use ethers::{
    contract::abigen,
    providers::{Provider, StreamExt, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, Chain as EChain, U256},
};

use relayer::Relayer;
use tokio::spawn;

mod relayer;

// contract interface / type safe.
abigen!(
    IBridge,
    r#"[
        function thisChain() external view returns (uint256) 
        event BridgeEth(address sender, uint256 amount, uint256 targetChain)
    ]"#,
);

// mainnet
const BRIDGE_ADDRESS: &str = "0x8ff7d3782c1621d0E6B67c7891cDbeEC3658a49d";

// optimism
const OPTIMISM_ADDRESS: &str = "0xCf389205330c476E3e8fDC689090312eB0dB397d";

enum Chain {
    Mainnet,
    Optimism,
}

async fn get_provider(chain: Chain) -> Provider<Ws> {
    let url: String;

    match chain {
        Chain::Mainnet => url = env::var("ETH_URL").unwrap(),
        Chain::Optimism => url = env::var("OP_URL").unwrap(),
    }
    let ws = Ws::connect(url).await.unwrap();
    Provider::new(ws)
}

fn get_wallet() -> LocalWallet {
    let pk = env::var("PK").unwrap();
    pk.parse().unwrap()
}

async fn listen_optimism() {
    let op_provider = get_provider(Chain::Optimism).await;
    let eth_provider = get_provider(Chain::Mainnet).await;
    let wallet: LocalWallet = get_wallet();
    let relayer = Relayer::new(wallet, eth_provider);
    let contract_address = Address::from_str(OPTIMISM_ADDRESS).unwrap();
    let contract = IBridge::new(contract_address, Arc::new(op_provider));
    let events = contract.event::<BridgeEthFilter>();
    let mut stream = events.stream().await.unwrap();

    println!("wallet relayer: {}", &relayer.get_address());
    println!("listening on optimism");

    while let Some(Ok(f)) = stream.next().await {
        println!("new transaction");
        let sender = f.sender;
        let amount = f.amount;
        let chain = f.target_chain;

        if chain == U256::from(1) {
            relayer.send_transaction(sender, amount).await.unwrap();
        } else {
            panic!("something weird happened")
        }
    }
}

async fn listen_ethereum() {
    let op_provider = get_provider(Chain::Optimism).await;
    let eth_provider = get_provider(Chain::Mainnet).await;
    let wallet: LocalWallet = get_wallet();
    let wallet = wallet.with_chain_id(EChain::Optimism);
    let relayer = Relayer::new(wallet, op_provider);
    let contract_address = Address::from_str(BRIDGE_ADDRESS).unwrap();
    let contract = IBridge::new(contract_address, Arc::new(eth_provider));
    let events = contract.event::<BridgeEthFilter>();
    let mut stream = events.stream().await.unwrap();

    println!("listening on Ethereum");
    println!("wallet relayer: {}", &relayer.get_address());

    while let Some(Ok(f)) = stream.next().await {
        println!("new transaction");
        let sender = f.sender;
        let amount = f.amount;
        let chain = f.target_chain;

        if chain == U256::from(420) {
            relayer.send_transaction(sender, amount).await.unwrap();
        } else {
            panic!("something weird happened")
        }
    }
}

#[tokio::main]
async fn main() {
    let optimism = spawn(listen_optimism());
    let ethereum = spawn(listen_ethereum());

    // We run both functions concurrently.
    tokio::try_join!(optimism, ethereum).unwrap();
}
