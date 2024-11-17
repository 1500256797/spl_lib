use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use std::str::FromStr;

#[test]
fn test_gengrate_mint_acount_address() {
    // 需要计算出 代币铸币账户的 ata 地址 、 债券曲线账户的地址 以及 关联债券曲线账户的地址
    let mint_address = Pubkey::from_str("CcQWG2M56Z1ESomovmzjvNPuDXBjHype7PoQPP2Zpump").unwrap();
    let pumpfun_program = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();
    let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let expected_bound_curve_address =
        Pubkey::from_str("Ab4DiSUzi4tHLkE2W1k4W24mvmFoxsMvKCcpfNixNTJF").unwrap();
    let associated_token_program =
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
    let bonding_curve_seed = "bonding-curve";
    let bonding_curve_address = Pubkey::find_program_address(
        &[
            bonding_curve_seed.as_bytes(),
            mint_address.to_bytes().as_slice(),
        ],
        &pumpfun_program,
    )
    .0;
    assert_eq!(
        bonding_curve_address.to_string(),
        expected_bound_curve_address.to_string()
    );
    // 计算出 债券曲线账户的地址
    let bound_curve_address_ata = Pubkey::find_program_address(
        &[
            &bonding_curve_address.to_bytes(),
            &token_program.to_bytes(),
            &mint_address.to_bytes(),
        ],
        &associated_token_program,
    )
    .0;
    assert_eq!(
        bound_curve_address_ata.to_string(),
        "YhTHuJANfML4Zd54mpDpCXYCeJp7qjPUJhpQoX8d6BT"
    );
}

#[test]
fn test_generate_ata_address() {
    //Msj 发一笔0.00002 token 到Zq3x
    let from_wallet_address = "3w1iMvjKGxpbGaaSekNUsZBcVKERg2BCsUZMGrjcTMsj"; //
    let receive_wallet_address = "9FE5ttsHschGHSyhun8N1X3R9KXupcbbojosCFWoZq3x";
    let spl_token_program = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"; // is legacy not spl_token 2022
    let token_mint_address = "MNDEFzGvMt87ueuHvVU9VcTqsAP5b3fTGPsHuuPA5ey";
    let associated_token_program = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
    // 分别计算发送方ata账号地址 和 接收方ata账号地址
    let from_wallet_address = Pubkey::from_str(from_wallet_address).unwrap();
    let receive_wallet_address = Pubkey::from_str(receive_wallet_address).unwrap();
    let spl_token_program = Pubkey::from_str(spl_token_program).unwrap();
    let token_mint_address = Pubkey::from_str(token_mint_address).unwrap();
    let associated_token_program = Pubkey::from_str(associated_token_program).unwrap();

    let from_associated_token_address = get_associated_token_address_with_program_id(
        &from_wallet_address,
        &token_mint_address,
        &spl_token_program,
    );

    let to_associated_token_address = get_associated_token_address_with_program_id(
        &receive_wallet_address,
        &token_mint_address,
        &spl_token_program,
    );
    // seed = [wallet_address,spl_token_program,token_mint]
    // must be order in the array
    let from_ata_address = Pubkey::find_program_address(
        &[
            &from_wallet_address.to_bytes(),
            &spl_token_program.to_bytes(),
            &token_mint_address.to_bytes(),
        ],
        &associated_token_program,
    )
    .0;
    let receive_ata_address = Pubkey::find_program_address(
        &[
            &receive_wallet_address.to_bytes(),
            &spl_token_program.to_bytes(),
            &token_mint_address.to_bytes(),
        ],
        &associated_token_program,
    )
    .0;

    assert_eq!(
        "DNF1LVkvg4ZiefjwMBiHdX5RTt4BZhLzN6H9h7rm719b",
        from_ata_address.to_string()
    );
    assert_eq!(
        "39jGmWURj2NsLUy6XHeDcPTQrmTTBeeZF4PMFzpuqUdg",
        receive_ata_address.to_string()
    );

    assert_eq!(
        "DNF1LVkvg4ZiefjwMBiHdX5RTt4BZhLzN6H9h7rm719b",
        from_associated_token_address.to_string()
    );
    assert_eq!(
        "39jGmWURj2NsLUy6XHeDcPTQrmTTBeeZF4PMFzpuqUdg",
        to_associated_token_address.to_string()
    );
    // https://solscan.io/tx/5y8mMKQzcyqdC2Ch7zY9jgatamoSPQN8c5z3Yyfokt6ojfRzaGWTVArWjZ8aptPSTiJUs6ahYzQyUncFHffzFwwy
}

#[test]
fn test_generate_usdc_ata_address() {
    //Msj 发一笔0.00001 usdc  到Zq3x
    let from_wallet_address = "3w1iMvjKGxpbGaaSekNUsZBcVKERg2BCsUZMGrjcTMsj";
    let receive_wallet_address = "9FE5ttsHschGHSyhun8N1X3R9KXupcbbojosCFWoZq3x";
    let spl_token_program = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"; // is legacy not spl_token 2022
    let token_mint_address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let associated_token_program = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
    // 分别计算发送方ata账号地址 和 接收方ata账号地址
    let from_wallet_address = Pubkey::from_str(from_wallet_address).unwrap();
    let receive_wallet_address = Pubkey::from_str(receive_wallet_address).unwrap();
    let spl_token_program = Pubkey::from_str(spl_token_program).unwrap();
    let token_mint_address = Pubkey::from_str(token_mint_address).unwrap();
    let associated_token_program = Pubkey::from_str(associated_token_program).unwrap();

    let from_associated_token_address = get_associated_token_address_with_program_id(
        &from_wallet_address,
        &token_mint_address,
        &spl_token_program,
    );

    let to_associated_token_address = get_associated_token_address_with_program_id(
        &receive_wallet_address,
        &token_mint_address,
        &spl_token_program,
    );
    // seed = [wallet_address,spl_token_program,token_mint]
    // must be order in the array
    let from_ata_address = Pubkey::find_program_address(
        &[
            &from_wallet_address.to_bytes(),
            &spl_token_program.to_bytes(),
            &token_mint_address.to_bytes(),
        ],
        &associated_token_program,
    )
    .0;
    let receive_ata_address = Pubkey::find_program_address(
        &[
            &receive_wallet_address.to_bytes(),
            &spl_token_program.to_bytes(),
            &token_mint_address.to_bytes(),
        ],
        &associated_token_program,
    )
    .0;

    assert_eq!(
        "GrfQTEskA8ZP2eNorbRogpw5DFGNEHBiZGHE2EiGHDqm",
        from_ata_address.to_string()
    );
    assert_eq!(
        "251p5FGaW787Jcrj6KYDSLY5tMsvCrFt68kjoS1Y2QjJ",
        receive_ata_address.to_string()
    );

    assert_eq!(
        "GrfQTEskA8ZP2eNorbRogpw5DFGNEHBiZGHE2EiGHDqm",
        from_associated_token_address.to_string()
    );
    assert_eq!(
        "251p5FGaW787Jcrj6KYDSLY5tMsvCrFt68kjoS1Y2QjJ",
        to_associated_token_address.to_string()
    );
    //https://solscan.io/tx/5pZYDdDkuakkhku9V4eifvm4SeAZ4tzQS8Cq5w2sVZVKq2636BWWjYhgpXWeAc2y1gSjBF76ZwZt1Nq3DNgw3ZQ4
}

#[test]
fn test_cal_receive_wallet_addr_from_receive_ata() {
    // 根据 ata地址和token mint地址 逆向计算出 收款钱包地址
    // https://solscan.io/tx/5qC3xrqbUFFyRDndjz7h7fgdRWmirykGANtt52pA2EGBLDfuzFJwLdfkSrFbV2Pbu91Uj6p4xuEBq9CraR9bdP8U
}
