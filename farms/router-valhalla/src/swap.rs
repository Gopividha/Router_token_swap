//! Swap tokens with the Orca pool instruction

use {
    solana_farm_sdk::program::{account, protocol::orca},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
        program_error::ProgramError,
    },
    spl_token_swap::instruction,
};

pub fn swap(
    accounts: &[AccountInfo],
    token_a_in: u64,
    token_b_in: u64,
    min_token_out: u64,
) -> ProgramResult {
    msg!("Processing AmmInstruction::Swap");


    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        user_account,
        user_token_a_account,
        user_token_b_account,
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
        


        let (amount_in, mut minimum_amount_out) = orca::get_pool_swap_amounts(
            pool_token_a_account,
            pool_token_b_account,
            token_a_in,
            token_b_in,
        )?;
        

        let data = instruction::Swap {
            amount_in,
            minimum_amount_out,
        };

        

        if token_a_in == 0 {
          
            let instruction = instruction::swap(
                pool_program_id.key,
                &spl_token::id(),
                amm_id.key,
                amm_authority.key,
                user_account.key,
                user_token_b_account.key,
                pool_token_b_account.key,
                pool_token_a_account.key,
                user_token_a_account.key,
                lp_token_mint.key,
                fees_account.key,
                None,
                data,
            )?;
            invoke(&instruction, accounts)?;

           
        } else {
          

            let instruction = instruction::swap(
                pool_program_id.key,
                &spl_token::id(),
                amm_id.key,
                amm_authority.key,
                user_account.key,
                user_token_a_account.key,
                pool_token_a_account.key,
                pool_token_b_account.key,
                user_token_b_account.key,
                lp_token_mint.key,
                fees_account.key,
                None,
                data,
            )?;
            invoke(&instruction, accounts)?;

           
        }
    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    msg!("AmmInstruction::Swap complete");
    Ok(())
}
