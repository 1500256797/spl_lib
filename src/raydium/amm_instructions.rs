use anchor_client::Client;
use anchor_client::Cluster;
use anyhow::anyhow;
use anyhow::Result;
use solana_sdk::signature::Keypair;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar};

use raydium_cp_swap::accounts as raydium_cp_accounts;
use raydium_cp_swap::instruction as raydium_cp_instructions;

use std::rc::Rc;

pub const AMM_CONFIG_SEED: &str = "amm_config";
/// Seed to derive account address and signature
pub const OBSERVATION_SEED: &str = "observation";
// Number of ObservationState element
pub const OBSERVATION_NUM: usize = 100;
pub const OBSERVATION_UPDATE_DURATION_DEFAULT: u64 = 15;
/// Seed to derive account address and signature
pub const POOL_SEED: &str = "pool";
pub const POOL_LP_MINT_SEED: &str = "pool_lp_mint";
pub const POOL_VAULT_SEED: &str = "pool_vault";

pub const AUTH_SEED: &str = "vault_and_lp_mint_auth_seed";
fn read_keypair_file(s: &str) -> Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| anyhow!("failed to read keypair from {}", s))
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    http_url: String,
    ws_url: String,
    payer_path: String,
    admin_path: String,
    raydium_cp_program: Pubkey,
    slippage: f64,
}

pub fn initialize_pool_instr(
    config: &ClientConfig,
    token_0_mint: Pubkey,
    token_1_mint: Pubkey,
    token_0_program: Pubkey,
    token_1_program: Pubkey,
    user_token_0_account: Pubkey,
    user_token_1_account: Pubkey,
    create_pool_fee: Pubkey,
    init_amount_0: u64,
    init_amount_1: u64,
    open_time: u64,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.raydium_cp_program)?;

    let amm_config_index = 0u16;
    let (amm_config_key, __bump) = Pubkey::find_program_address(
        &[AMM_CONFIG_SEED.as_bytes(), &amm_config_index.to_be_bytes()],
        &program.id(),
    );

    let (pool_account_key, __bump) = Pubkey::find_program_address(
        &[
            POOL_SEED.as_bytes(),
            amm_config_key.to_bytes().as_ref(),
            token_0_mint.to_bytes().as_ref(),
            token_1_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (authority, __bump) = Pubkey::find_program_address(&[AUTH_SEED.as_bytes()], &program.id());
    let (token_0_vault, __bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_0_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (token_1_vault, __bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_1_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (lp_mint_key, __bump) = Pubkey::find_program_address(
        &[
            POOL_LP_MINT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (observation_key, __bump) = Pubkey::find_program_address(
        &[
            OBSERVATION_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );

    let instructions = program
        .request()
        .accounts(raydium_cp_accounts::Initialize {
            creator: program.payer(),
            amm_config: amm_config_key,
            authority,
            pool_state: pool_account_key,
            token_0_mint,
            token_1_mint,
            lp_mint: lp_mint_key,
            creator_token_0: user_token_0_account,
            creator_token_1: user_token_1_account,
            creator_lp_token: spl_associated_token_account::get_associated_token_address(
                &program.payer(),
                &lp_mint_key,
            ),
            token_0_vault,
            token_1_vault,
            create_pool_fee,
            observation_state: observation_key,
            token_program: spl_token_2022::id(),
            token_0_program,
            token_1_program,
            associated_token_program: spl_associated_token_account::id(),
            system_program: system_program::id(),
            rent: sysvar::rent::id(),
        })
        .args(raydium_cp_instructions::Initialize {
            init_amount_0,
            init_amount_1,
            open_time,
        })
        .instructions()?;
    Ok(instructions)
}
