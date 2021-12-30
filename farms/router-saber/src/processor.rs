//! Saber router implementation.

use {
    crate::{
        add_liquidity::add_liquidity, remove_liquidity::remove_liquidity,
        swap::swap,
    },
    solana_farm_sdk::{instruction::amm::AmmInstruction, log::sol_log_params_short},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, log::sol_log_compute_units, msg,
        pubkey::Pubkey,
    },
};

/// Program's entrypoint.
///
/// # Arguments
/// * `program_id` - Public key of the router.
/// * `accounts` - Accounts, see particular instruction handler for the list.
/// * `instructions_data` - Packed AmmInstruction.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Valhalla router entrypoint");
   
    // Read and unpack instruction data
    let instruction = AmmInstruction::unpack(instruction_data)?;

    match instruction {
        //Deposite All_token_types in Token_swap_pool
        AmmInstruction::AddLiquidity {
            max_token_a_amount,
            max_token_b_amount,
            pool_token_amount,
        } => add_liquidity(accounts, max_token_a_amount, max_token_b_amount,pool_token_amount)?,
        //Withdraw All_token_types from Token_swap_pool
        AmmInstruction::RemoveLiquidity { 
            pool_token_amount,
            minimum_token_a_amount,
            minimum_token_b_amount,

        } => remove_liquidity(accounts, pool_token_amount,minimum_token_a_amount,minimum_token_b_amount)?,
        //swapping of token A to B or Bto A .....
        AmmInstruction::Swap {
            token_a_amount_in,
            token_b_amount_in,
            min_token_amount_out,
        } => swap(
            accounts,
            token_a_amount_in,
            token_b_amount_in,
            min_token_amount_out,
        )?,
       
    }

    msg!("Valhalla router end of instruction");
    Ok(())
}
