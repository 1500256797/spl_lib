use mpl_token_metadata::{ instructions::TransferV1Builder};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program, sysvar};
use mpl_token_metadata::{accounts::Metadata, instructions::MintV1Builder};
pub fn process_transfer_to(
    signer: &dyn Signer,
    mint_pubkey: Pubkey,
    receiver_pubkey: Pubkey,
    amount: u64,
) -> Instruction {
    let (metadata, _) = Metadata::find_pda(&mint_pubkey);
    let receiver_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &receiver_pubkey,
        &mint_pubkey,
        &spl_token_2022::id(),
    );
    let signer_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &signer.pubkey(),
        &mint_pubkey,
        &spl_token_2022::id(),
    );




    let transfer_to_ix = TransferV1Builder::new()
        .token(signer_ata)
        .token_owner(signer.pubkey())
        .destination_token(receiver_ata)
        .destination_owner(receiver_pubkey)
        .metadata(metadata)
        .mint(mint_pubkey)
        .amount(amount)
        .authority(signer.pubkey())
        .payer(signer.pubkey())
        .spl_token_program(spl_token_2022::ID)
        .spl_ata_program(spl_associated_token_account::ID)
        .system_program(system_program::ID)
        .sysvar_instructions(sysvar::instructions::ID)
        .instruction();
    transfer_to_ix
}
