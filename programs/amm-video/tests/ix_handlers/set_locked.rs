use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
};

pub fn create_set_locked_ix(signer: &Keypair, config: Pubkey, locked: bool) -> Instruction {
    Instruction::new_with_bytes(
        amm_video::id(),
        &amm_video::instruction::SetLocked { locked }.data(),
        amm_video::accounts::SetLocked {
            authority: signer.pubkey(),
            config,
        }
        .to_account_metas(None),
    )
}