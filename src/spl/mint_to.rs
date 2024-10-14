use mpl_token_metadata::{accounts::Metadata, instructions::MintV1Builder};
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program, sysvar,
};

pub fn process_mint_to(
    signer: &dyn Signer,
    mint_pubkey: Pubkey,
    receiver_pubkey: Pubkey,
    amount: u64,
) -> Instruction {
    let receiver_ata = spl_associated_token_account::get_associated_token_address_with_program_id(
        &receiver_pubkey,
        &mint_pubkey,
        &spl_token_2022::id(),
    );
    let (metadata, _) = Metadata::find_pda(&mint_pubkey);

    let mint_to_ix = MintV1Builder::new()
        .token(receiver_ata)
        .token_owner(Option::<Pubkey>::Some(receiver_pubkey))
        .metadata(metadata)
        .mint(mint_pubkey)
        .amount(amount)
        .authority(signer.pubkey())
        .payer(signer.pubkey())
        .system_program(system_program::ID)
        .sysvar_instructions(sysvar::instructions::ID)
        .spl_token_program(spl_token_2022::ID)
        .spl_ata_program(spl_associated_token_account::ID)
        .instruction();
    mint_to_ix
}
