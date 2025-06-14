use super::types::SwapInfo;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyInfo {
    pub discriminator: u64,
    pub token_amount: u64,
    pub sol_amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellInfo {
    pub discriminator: u64,
    pub token_amount: u64,
    pub sol_amount: u64,
}

impl From<SwapInfo> for BuyInfo {
    fn from(buy: SwapInfo) -> Self {
        Self {
            discriminator: 16927863322537952870,
            token_amount: buy.token_amount,
            sol_amount: buy.sol_amount,
        }
    }
}

impl From<SwapInfo> for SellInfo {
    fn from(sell: SwapInfo) -> Self {
        Self {
            discriminator: 12502976635542562355,
            token_amount: sell.token_amount,
            sol_amount: sell.sol_amount,
        }
    }
}

impl BuyInfo {
    pub fn to_buffer(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer)?;
        Ok(buffer)
    }
}

impl SellInfo {
    pub fn to_buffer(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer)?;
        Ok(buffer)
    }
}
