use mpl_token_metadata::accounts::{MasterEdition, Metadata};
use mpl_token_metadata::instructions::{CreateV1, CreateV1Builder, CreateV1InstructionArgs};
use mpl_token_metadata::types::{PrintSupply, TokenStandard};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_sdk::{instruction::Instruction, native_token::LAMPORTS_PER_SOL};
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    system_instruction,
};
use spl_token_2022::state::Mint;
// concept
// mint account: https://solana.com/docs/core/tokens#mint-account
// token program: https://solana.com/docs/core/tokens#token-program
// token account: token account record a wallet token balance . some times use ata to generate a token account
// A Mint Account represents a unique token on the network and stores global metadata such as total supply.
// A Token Account tracks individual ownership of tokens for a specific mint account.
// An Associated Token Account is a Token Account created with an address derived from the owner's and mint account's addresses.
// https://spl.solana.com/token

pub fn prepare_deploy_token_with_metadata_instructions(
    mint_account: &Keypair,
    wallet_keypair: &Keypair,
) -> Vec<Instruction> {
    let solana_client =
        solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
    let mint_account = mint_account.pubkey();
    let mint_rent = solana_client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .unwrap();
    // create empty account ( space )
    let create_account_space_ix = system_instruction::create_account(
        &wallet_keypair.pubkey(),
        &mint_account,
        mint_rent as u64,
        Mint::LEN as u64,
        &spl_token_2022::id(),
    );

    // initialize mint account (init this space)
    let mint_account_init_ix = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::id(),
        &mint_account,
        &wallet_keypair.pubkey(),
        Some(&wallet_keypair.pubkey()),
        9,
    )
    .unwrap();

    // update mint account detail
    let payer = wallet_keypair.pubkey();
    let master_edition_pda = MasterEdition::find_pda(&mint_account);
    // instruction args
    let create_ix = CreateV1Builder::new()
            .metadata(Metadata::find_pda(&mint_account).0)
            .master_edition(Some(master_edition_pda.0))
            .mint(mint_account, false)
            .authority(payer)
            .payer(payer)
            .update_authority(payer, true)
            .is_mutable(true)
            .primary_sale_happened(false)
            .name(String::from("NewsMeMe"))
            .uri(String::from("https://white-historical-basilisk-887.mypinata.cloud/ipfs/QmVd6xVRqg9sJQP1zkUVizZ7jah6zD7j6fSPn9F7MRjZMo"))
            .seller_fee_basis_points(500)
            .token_standard(TokenStandard::Fungible)
            .print_supply(PrintSupply::Limited(10000000000000000))
            .instruction();

    vec![create_account_space_ix, mint_account_init_ix, create_ix]
}

pub fn prepare_mint_token_instruction(
    mint_account: &Pubkey,
    wallet_keypair: &Keypair,
    amount: u64,
    need_create_ata: bool,
) -> Vec<Instruction> {
    // The Associated Token Program uses Cross Program Invocations to handle:
    // Invoking the System Program to create a new account using the provided PDA as the address of the new account
    // Invoking the Token Program to initialize the Token Account data for the new account.
    // 使用 create_associated_token_account = system_instruction::create_account + spl_token_2022::instruction::initialize_account3 + .....
    // https://explorer.solana.com/tx/58EDj9im952aomeiqa6iWH7wWA9uyQtWAHRJDXUE4by63jckAHMbMWkoAxsTF1JBvF8t2TWPvGQ9fCTpqbyJ8UjK?cluster=devnet
    let ata_account = spl_associated_token_account::get_associated_token_address_with_program_id(
        &wallet_keypair.pubkey(),
        &mint_account,
        &spl_token_2022::id(),
    );
    println!("ata_account: {}", ata_account.to_string());

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &wallet_keypair.pubkey(),
        &wallet_keypair.pubkey(),
        &mint_account,
        &spl_token_2022::id(),
    );

    // mint amount token to ata account
    let mint_token_ix = spl_token_2022::instruction::mint_to(
        &spl_token_2022::id(),
        &mint_account,
        &ata_account,
        &wallet_keypair.pubkey(),
        vec![&wallet_keypair.pubkey()].as_slice(),
        amount,
    )
    .unwrap();

    if need_create_ata {
        vec![create_ata_ix, mint_token_ix]
    } else {
        vec![mint_token_ix]
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use mpl_token_metadata::{
        accounts::{MasterEdition, Metadata},
        instructions::CreateV1Builder,
        types::{PrintSupply, TokenStandard},
    };
    use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
    use spl_token_2022::id;

    #[test]
    fn test_mint_token_instruction() {
        // https://explorer.solana.com/address/Bm1mYeDfqyVdLCuvSTBpJjSnZApftetPQJ9MYHqfFL3Q/tokens?cluster=devnet
        let wallet_keypair = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        let mint_account =
            Pubkey::from_str("5LdzEFRMQy2SCf2SD4TXkRao8ELh7FZAzqQGia5DNxKE").unwrap();
        let amount = 999 * 10_u64.pow(9);
        let instructions =
            prepare_mint_token_instruction(&mint_account, &wallet_keypair, amount, false);
        println!("instructions: {}", instructions.len());

        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&wallet_keypair.pubkey()),
            &[&wallet_keypair],
            solana_client.get_latest_blockhash().unwrap(),
        );

        solana_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .unwrap();
    }
    #[test]
    fn test_deploy_token_with_metadata() {
        let mint_account = Keypair::new();
        let wallet_keypair = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        let instructions =
            prepare_deploy_token_with_metadata_instructions(&mint_account, &wallet_keypair);
        println!("instructions: {}", instructions.len());
        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&wallet_keypair.pubkey()),
            &[&wallet_keypair, &mint_account],
            solana_client.get_latest_blockhash().unwrap(),
        );

        solana_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .unwrap();
    }
    #[test]
    fn test_new_solana_test_account() {
        // ~ solana-keygen new -o /Users/jianjianjianjian/.config/solana/id.json
        // Generating a new keypair
        //
        // For added security, enter a BIP39 passphrase
        //
        // NOTE! This passphrase improves security of the recovery seed phrase NOT the
        // keypair file itself, which is stored as insecure plain text
        //
        // BIP39 Passphrase (empty for none):
        //
        //     Wrote new keypair to /Users/jianjianjianjian/.config/solana/id.json
        //     ============================================================================
        //     pubkey: 9WYirnyBy8RMBuoatC9yVCRQZ6AYpKpMyKmr2TrjypCG
        //     ============================================================================
        //     Save this seed phrase and your BIP39 passphrase to recover your new keypair:
        //     antenna air ski border destroy fold gather put cheese exercise romance nerve
        //     ============================================================================
    }
    #[test]
    fn test_init_mint_account() {
        let wallet_keypair = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        let pubkey = wallet_keypair.pubkey().to_string();
        assert_eq!("9WYirnyBy8RMBuoatC9yVCRQZ6AYpKpMyKmr2TrjypCG", pubkey);
        // https://explorer.solana.com/address/9WYirnyBy8RMBuoatC9yVCRQZ6AYpKpMyKmr2TrjypCG?cluster=devnet
        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
        let mint_account = Keypair::new();
        let mint_rent = solana_client
            .get_minimum_balance_for_rent_exemption(Mint::LEN)
            .unwrap();
        // create empty account ( space )
        let create_account_space_ix = system_instruction::create_account(
            &wallet_keypair.pubkey(),
            &mint_account.pubkey(),
            mint_rent as u64,
            Mint::LEN as u64,
            &spl_token_2022::id(),
        );

        let default_decimals = 9;
        // initialize mint account (init this space)
        let mint_account_init_ix = spl_token_2022::instruction::initialize_mint(
            &spl_token_2022::id(),
            &mint_account.pubkey(),
            &wallet_keypair.pubkey(),
            Some(&wallet_keypair.pubkey()),
            default_decimals,
        )
        .unwrap();
        let recent_blockhash = solana_client.get_latest_blockhash().unwrap();

        // Invoke the System Program to create a new account with enough space for the Mint Account data and then transfer ownership to the Token Program.
        // Invoke the Token Program to initialize the data of the new account as a Mint Account
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[create_account_space_ix, mint_account_init_ix],
            Some(&wallet_keypair.pubkey()),
            &[&mint_account, &wallet_keypair],
            recent_blockhash,
        );

        solana_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .unwrap();
        // https://explorer.solana.com/address/FTdha6vLcKgvbiMK66SxY7X9PPEyRNBWRyYsKGEiEgW3/instructions?cluster=devnet
        println!(
            "SPL Token mint account with {} decimals created successfully:\n{}",
            9,
            mint_account.pubkey().to_string()
        );
    }

    #[test]
    fn test_update_mint_account_detail() {
        let mint_account =
            Pubkey::from_str("FTdha6vLcKgvbiMK66SxY7X9PPEyRNBWRyYsKGEiEgW3").unwrap();
        let wallet_keypair = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        let payer = wallet_keypair.pubkey();
        let metadata_pda = Metadata::find_pda(&mint_account);
        let master_edition_pda = MasterEdition::find_pda(&mint_account);
        // instruction args
        let create_ix = CreateV1Builder::new()
            .metadata(metadata_pda.0)
            .master_edition(Some(master_edition_pda.0))
            .mint(mint_account, false)
            .authority(payer)
            .payer(payer)
            .update_authority(payer, true)
            .is_mutable(true)
            .primary_sale_happened(false)
            .name(String::from("News"))
            .uri(String::from("https://white-historical-basilisk-887.mypinata.cloud/ipfs/QmVd6xVRqg9sJQP1zkUVizZ7jah6zD7j6fSPn9F7MRjZMo"))
            .seller_fee_basis_points(500)
            .token_standard(TokenStandard::Fungible)
            .print_supply(PrintSupply::Limited(10000000000000000))
            .instruction();
        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());

        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&wallet_keypair.pubkey()),
            &[&wallet_keypair],
            solana_client.get_latest_blockhash().unwrap(),
        );
        // send transaction to solana network
        let result = solana_client.send_and_confirm_transaction(&transaction);
        match result {
            Ok(sig) => println!("Transaction successful: {}", sig),
            Err(e) => println!("Transaction failed: {}", e),
        }
        // https://solscan.io/token/EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm#metadata
        // https://explorer.solana.com/address/FTdha6vLcKgvbiMK66SxY7X9PPEyRNBWRyYsKGEiEgW3/metadata?cluster=devnet
    }

    #[test]
    fn test_init_token_account_mint_token() {
        let wallet_keypair = Keypair::from_bytes(&[
            209, 77, 194, 225, 64, 117, 226, 133, 133, 78, 162, 100, 82, 186, 248, 218, 177, 68,
            141, 213, 3, 127, 245, 190, 4, 30, 250, 40, 254, 7, 32, 26, 126, 111, 52, 235, 27, 57,
            65, 27, 193, 119, 167, 112, 155, 211, 191, 153, 125, 177, 216, 172, 95, 17, 157, 120,
            98, 170, 226, 75, 220, 140, 11, 41,
        ])
        .unwrap();
        let pubkey = wallet_keypair.pubkey().to_string();
        assert_eq!("9WYirnyBy8RMBuoatC9yVCRQZ6AYpKpMyKmr2TrjypCG", pubkey);

        // mint account come from https://explorer.solana.com/address/FTdha6vLcKgvbiMK66SxY7X9PPEyRNBWRyYsKGEiEgW3/instructions?cluster=devnet
        let mint_account =
            Pubkey::from_str("FTdha6vLcKgvbiMK66SxY7X9PPEyRNBWRyYsKGEiEgW3").unwrap();
        // The Associated Token Program uses Cross Program Invocations to handle:
        // Invoking the System Program to create a new account using the provided PDA as the address of the new account
        // Invoking the Token Program to initialize the Token Account data for the new account.
        let ata_account =
            spl_associated_token_account::get_associated_token_address_with_program_id(
                &wallet_keypair.pubkey(),
                &mint_account,
                &id(),
            );
        println!("ata_account: {}", ata_account.to_string());
        let solana_client =
            solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());

        // 使用 create_associated_token_account = system_instruction::create_account + spl_token_2022::instruction::initialize_account3 + .....
        // https://explorer.solana.com/tx/58EDj9im952aomeiqa6iWH7wWA9uyQtWAHRJDXUE4by63jckAHMbMWkoAxsTF1JBvF8t2TWPvGQ9fCTpqbyJ8UjK?cluster=devnet
        let create_ata_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &wallet_keypair.pubkey(),
                &wallet_keypair.pubkey(),
                &mint_account,
                &id(),
            );

        // mint 100 token
        let mint_100_token_ix = spl_token_2022::instruction::mint_to(
            &id(),
            &mint_account,
            &ata_account,
            &wallet_keypair.pubkey(),
            vec![&wallet_keypair.pubkey()].as_slice(),
            100,
        )
        .unwrap();

        // construct  transaction and sign it
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[create_ata_ix, mint_100_token_ix],
            Some(&wallet_keypair.pubkey()),
            &[&wallet_keypair],
            solana_client.get_latest_blockhash().unwrap(),
        );

        // send transaction to solana network
        let result = solana_client.send_and_confirm_transaction(&transaction);
        match result {
            Ok(sig) => println!("Transaction successful: {}", sig),
            Err(e) => println!("Transaction failed: {}", e),
        }
        // https://explorer.solana.com/address/G26feaTmREY7riM2Lw52zdaMmjGPxL9LbgnFcpdop8fN?cluster=devnet
        // 58EDj9im952aomeiqa6iWH7wWA9uyQtWAHRJDXUE4by63jckAHMbMWkoAxsTF1JBvF8t2TWPvGQ9fCTpqbyJ8UjK
    }
}
