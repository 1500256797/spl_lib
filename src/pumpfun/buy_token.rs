use serde::{Deserialize, Serialize};
use std::str::FromStr;

use bitcoin::base58;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_instruction};

use derive_builder::Builder;

#[derive(Default, Builder, Debug)]
pub struct BuyAccounts {
    /// 全局状态账户
    #[builder(default = "String::from(\"4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf\")")]
    pub global: String,

    /// 费用接收账户
    #[builder(default = "String::from(\"CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM\")")]
    pub fee_recipient: String,

    /// 代币铸币账户
    pub mint: String,

    /// 债券曲线账户
    pub bonding_curve: String,

    /// 关联债券曲线账户
    pub associated_bonding_curve: String,

    /// 用户的关联代币账户
    pub associated_user: String,

    /// 用户钱包账户
    pub user: String,

    /// 系统程序
    #[builder(default = "String::from(\"11111111111111111111111111111111\")")]
    pub system_program: String,

    /// 代币程序
    #[builder(default = "String::from(\"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA\")")]
    pub token_program: String,

    /// 租金账户
    #[builder(default = "String::from(\"SysvarRent111111111111111111111111111111111\")")]
    pub rent: String,

    /// 事件权限账户
    #[builder(default = "String::from(\"Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1\")")]
    pub event_authority: String,

    /// 程序账户
    #[builder(default = "String::from(\"6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P\")")]
    pub program: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyArgs {
    /// 购买代币数量
    pub amount: u64,
    /// 最大 SOL 支付成本
    pub max_sol_cost: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_buy_tx() {
        let accounts = BuyAccountsBuilder::default().build().unwrap();
        println!("{:?}", accounts);
    }
}
