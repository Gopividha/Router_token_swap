//! Unstake LP tokens from farm instruction

use {
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg, 
        program::invoke,
        program_error::ProgramError,
    },
    farm::instruction::withdraw,
};

pub fn unstake(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("Processing AmmInstruction::Unstake");
    msg!("amount {} ", amount);

    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        user_account,
        user_info_account,
        farm_state_account,
        user_lp_token_account,
        pda,
        pda_token_account,
        farm_lp_token_mint,
        token_program_id,
        farm_program_id,

        ] = accounts
    {
        msg!("accounts correct");
        let unstake_accounts = vec![
            AccountMeta::new(*user_account.key, true),
            AccountMeta::new(*user_info_account.key, false),
            AccountMeta::new(*farm_state_account.key, false),
            AccountMeta::new(*user_lp_token_account.key, false),
            AccountMeta::new_readonly(*pda.key, false),
            AccountMeta::new(*pda_token_account.key, false),
            AccountMeta::new_readonly(*farm_lp_token_mint.key, false),
            AccountMeta::new_readonly(*token_program_id.key, false),
            AccountMeta::new_readonly(*farm_program_id.key, false),
        ];
           
        let instruction = withdraw (
            user_account.key,
            user_info_account.key,
            farm_state_account.key,
            user_lp_token_account.key,
            pda.key,
            pda_token_account.key,
            farm_lp_token_mint.key,
            token_program_id.key,
            farm_program_id.key,
            amount,
        );
        msg!("berfore invoke");
        invoke(&instruction, accounts)?;

    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    msg!("AmmInstruction::Unstake complete");
    Ok(())
    
}
