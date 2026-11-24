use {
    anchor_lang::{
        solana_program::instruction::Instruction, system_program::ID as SYSTEM_PROGRAM_ID,
        InstructionData, ToAccountMetas,
    },
    anchor_spl::associated_token::{self, ID as ASSOCIATED_TOKEN_PROGRAM_ID},
    litesvm::LiteSVM,
    litesvm_token::spl_token::ID as TOKEN_PROGRAM_ID,
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
};

pub fn create_collect_fees_ix(
    mut _svm: &mut LiteSVM,
    payer: &Keypair,
    mint_x: Pubkey,
    mint_y: Pubkey,
    config: Pubkey,
) -> Instruction {
    let authority = payer.pubkey();
    let authority_x = associated_token::get_associated_token_address(&authority, &mint_x);
    let authority_y = associated_token::get_associated_token_address(&authority, &mint_y);
    let treasury_x = Pubkey::find_program_address(&[b"treasury", config.as_ref(), mint_x.as_ref()], &amm_video::id()).0;
    let treasury_y = Pubkey::find_program_address(&[b"treasury", config.as_ref(), mint_y.as_ref()], &amm_video::id()).0;
    Instruction::new_with_bytes(
        amm_video::id(),
        &amm_video::instruction::CollectFees {}
        .data(),
        amm_video::accounts::CollectFees {
            authority,
            mint_x,
            mint_y,
            config,
            treasury_x,
            treasury_y,
            authority_x,
            authority_y,        
            token_program: TOKEN_PROGRAM_ID,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
    )
}
