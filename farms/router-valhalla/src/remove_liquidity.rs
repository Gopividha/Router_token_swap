//! Remove liquidity from the Orca pool instruction

use {
    solana_farm_sdk::program::{account, protocol::orca},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
        program_error::ProgramError,
    },
    spl_token_swap::instruction,
};

pub fn remove_liquidity(accounts: &[AccountInfo], 
    pool_token_amount: u64,
    minimum_token_a_amount: u64,
    minimum_token_b_amount: u64,

) -> ProgramResult {
    msg!("Processing AmmInstruction::RemoveLiquidity");

    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        user_account,
        user_token_a_account,
        user_token_b_account,
        user_lp_token_account,
        pool_program_id,
        pool_token_a_account,
        pool_token_b_account,
        lp_token_mint,
        _spl_token_id,
        amm_id,
        amm_authority,
        fees_account
        ] = accounts
    {
       
        let lp_amount = if pool_token_amount > 0 {
            pool_token_amount
        } else {
            account::get_token_balance(user_lp_token_account)?
        };

        let (token_a_amount, token_b_amount) = orca::get_pool_withdrawal_amounts(
            pool_token_a_account,
            pool_token_b_account,
            lp_token_mint,
            lp_amount,
        )?;

        let data = instruction::WithdrawAllTokenTypes {
            pool_token_amount: lp_amount,
            minimum_token_a_amount: token_a_amount,
            minimum_token_b_amount: token_b_amount,
        };

        
        let instruction = instruction::withdraw_all_token_types(
            pool_program_id.key,
            &spl_token::id(),
            amm_id.key,
            amm_authority.key,
            user_account.key,
            lp_token_mint.key,
            fees_account.key,
            user_lp_token_account.key,
            pool_token_a_account.key,
            pool_token_b_account.key,
            user_token_a_account.key,
            user_token_b_account.key,
            data,
        )?;

        invoke(&instruction, accounts)?;

    
    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    msg!("AmmInstruction::RemoveLiquidity complete");
    Ok(())
}
