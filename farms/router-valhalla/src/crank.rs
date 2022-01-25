//! Vault Crank instruction handler

use {
    crate::{
        val_crank1::val_crank1,
    },
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    },
};
pub struct CrankInstruction;


impl CrankInstruction {
    pub fn crank(accounts: &[AccountInfo], step: u64) -> ProgramResult {
        match step {
            1 => val_crank1(accounts),
           // 2 => val_crank2(accounts),
            _ => {
                msg!("Error: Invalid Crank step");
                Err(ProgramError::InvalidArgument)
            }
        }
    }
}
