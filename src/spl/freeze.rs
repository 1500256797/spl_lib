use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};
use spl_associated_token_account::get_associated_token_address;

pub fn process_freeze_account(
    signer: &dyn Signer,
    mint_pubkey: Pubkey,
    receiver_pubkey: Pubkey,
) -> Instruction {
    let receiver_ata = get_associated_token_address(&receiver_pubkey, &mint_pubkey);

    let freeze_ix = spl_token_2022::instruction::freeze_account(
        &spl_token_2022::ID,
        &receiver_ata,
        &mint_pubkey,
        &signer.pubkey(),
        &[&signer.pubkey()],
    )
    .unwrap();
    freeze_ix
}
