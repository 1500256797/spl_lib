use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};

pub fn process_unfreeze_account(
    signer: &dyn Signer,
    mint_pubkey: Pubkey,
    receiver_pubkey: Pubkey,
) -> Instruction {
    let receiver_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &receiver_pubkey,
        &mint_pubkey,
        &spl_token_2022::id(),
    );
    let unfreeze_ix = spl_token_2022::instruction::thaw_account(
        &spl_token_2022::ID,
        &receiver_ata,
        &mint_pubkey,
        &signer.pubkey(),
        &[&signer.pubkey()],
    )
    .unwrap();
    unfreeze_ix
}
