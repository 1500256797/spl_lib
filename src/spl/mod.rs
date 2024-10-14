pub mod create_spl_token;
pub mod freeze;
pub mod mint_to;
pub mod transfer_to;
pub mod unfreeze;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use solana_sdk::{
        pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
    };

    use crate::spl::mint_to::process_mint_to;

    use super::transfer_to::process_transfer_to;

    #[test]
    fn test_mint_to() {
        let signer = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        let mint_account =
            Pubkey::from_str("5LdzEFRMQy2SCf2SD4TXkRao8ELh7FZAzqQGia5DNxKE").unwrap();
        let receiver_account = signer.pubkey();
        let amount = 99999 * 10_u64.pow(9);
        let instruction = process_mint_to(&signer, mint_account, receiver_account, amount);

        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer],
            solana_client.get_latest_blockhash().unwrap(),
        );

        solana_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .unwrap();
    }



    #[test]
    fn test_transfer_to() {
        let signer = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        println!("signer: {}", signer.pubkey());
        let mint_account =
            Pubkey::from_str("5LdzEFRMQy2SCf2SD4TXkRao8ELh7FZAzqQGia5DNxKE").unwrap();
        let receiver_account =
            Pubkey::from_str("FNPYLsgYpJDUuDCiJmwuPSo2eKend71n8kp4cZBendfm").unwrap();
        let amount = 22 * 10_u64.pow(9);
        let instruction = process_transfer_to(&signer, mint_account, receiver_account, amount);

        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer],
            solana_client.get_latest_blockhash().unwrap(),
        );

        solana_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .unwrap();
    }
}
