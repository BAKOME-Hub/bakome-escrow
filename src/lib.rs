use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.first() {
        Some(&0) => { msg!("🔒 Bakome Escrow: Dépôt créé"); Ok(()) }
        Some(&1) => { msg!("✅ Bakome Escrow: Paiement libéré!"); Ok(()) }
        Some(&2) => { msg!("↩️ Bakome Escrow: Remboursement client"); Ok(()) }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
