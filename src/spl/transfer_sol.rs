use std::str::FromStr;

use bitcoin::base58;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_instruction};
pub fn transfer_sol(signer: &dyn Signer, recipient: Pubkey, amount: u64) -> Instruction {
    let transfer_sol_ix = system_instruction::transfer(&signer.pubkey(), &recipient, amount);
    transfer_sol_ix
}
pub fn get_address(pub_key: &String) -> String {
    let pubkey = hex::decode(pub_key).unwrap();
    if pubkey.len() != 32 {
        panic!("bad public key {:?}", pub_key);
    }
    base58::encode(pubkey.as_slice())
}

// get address sol balance
pub fn get_balance(address: &String) -> u64 {
    let client = solana_client::rpc_client::RpcClient::new(
        "https://api.mainnet-beta.solana.com".to_string(),
    );
    client
        .get_balance(&Pubkey::from_str(address).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Mnemonic;
    use solana_sdk::{
        derivation_path::DerivationPath,
        pubkey::Pubkey,
        signature::{self, Keypair},
        signer::{SeedDerivable, Signer},
        transaction::Transaction,
    };
    use std::str::FromStr;

    use crate::spl::transfer_sol::transfer_sol;

    #[test]
    fn test_get_sol_banlance() {
        let balance = get_balance(&"HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk".to_string());
        println!("{:?}", balance);
        // convert human readable
        let balance_humanreadable = balance as f64 / 1_000_000_000.0;
        println!("{:?}", balance_humanreadable);
    }

    #[test]
    fn test_transfer_sol_with_priv() {
        let key = "xxxxx";
        let signer = Keypair::from_base58_string(key);
        println!("{:?}", signer.pubkey());
        let balance = get_balance(&signer.pubkey().to_string());
        println!("{:?}", balance);
        let recipient = Pubkey::from_str("FLzoxtpBbnn5nGcyokN47Di2M3VJ7FaiUNGpeZWVRUgz").unwrap();
        assert_eq!(
            recipient.to_string(),
            "FLzoxtpBbnn5nGcyokN47Di2M3VJ7FaiUNGpeZWVRUgz"
        );
        let instruction = transfer_sol(&signer, recipient, (balance as f64 * 0.9) as u64);

        let solana_client = solana_client::rpc_client::RpcClient::new(
            "https://api.mainnet-beta.solana.com".to_string(),
        );
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer],
            solana_client.get_latest_blockhash().unwrap(),
        );

        let res = solana_client.send_and_confirm_transaction_with_spinner(&transaction);
        match res {
            Ok(signature) => {
                println!("{:?}", signature)
            }
            Err(e) => {
                println!("cannot send this tx , because {:?}", e)
            }
        }
    }

    #[test]
    fn test_transfer_sol() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_str(mnemonic).unwrap();

        let seed = mnemonic.to_seed("");
        // path = m/44'/501'/x'/0'
        let path = Some(DerivationPath::from_absolute_path_str("m/44'/501'/0'/0'").unwrap());
        let signer = Keypair::from_seed_and_derivation_path(&seed, path).unwrap();
        assert_eq!(
            "HAgk14JpMQLgt6rVgv7cBQFJWFto5Dqxi472uT3DKpqk",
            signer.pubkey().to_string()
        );
        let recipient = Pubkey::from_str("FLzoxtpBbnn5nGcyokN47Di2M3VJ7FaiUNGpeZWVRUgz").unwrap();
        assert_eq!(
            recipient.to_string(),
            "FLzoxtpBbnn5nGcyokN47Di2M3VJ7FaiUNGpeZWVRUgz"
        );
        let amount = 80000000;
        let instruction = transfer_sol(&signer, recipient, amount);

        let solana_client = solana_client::rpc_client::RpcClient::new(
            "https://api.mainnet-beta.solana.com".to_string(),
        );
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer],
            solana_client.get_latest_blockhash().unwrap(),
        );

        let res = solana_client.send_and_confirm_transaction_with_spinner(&transaction);
        match res {
            Ok(signature) => {
                println!("{:?}", signature)
            }
            Err(e) => {
                panic!("cannot send this tx , because {:?}", e)
            }
        }
    }
}
