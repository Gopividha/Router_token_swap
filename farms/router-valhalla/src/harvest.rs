use {
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg,
        program::invoke,
        program_error::ProgramError,
        program_pack::Pack,
    },
    farm::{
        instruction,
        state::{FarmMain,FarmState,FarmUserState},
    },
};
pub fn harvest(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Processing AmmInstruction::Harvest");
    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        user_account,
        user_info_account,
        farm_main_val_account,
        farm_state_account,
        user_reward_account,
        pda_main_account,
        reward_mint,
        _spl_token_id,
        farm_program_id,
        ] = accounts
    {
        let amount:u64=1200;
        msg!("enter");


        let farm_user_data = FarmUserState::unpack(&user_info_account.try_borrow_data()?)?;
        //let farm_state_account = farm_user_data.farm_state;

        let farm_main_data = FarmMain::unpack(&farm_main_val_account.try_borrow_data()?)?;
        let farm_state_data = FarmState::unpack(&farm_state_account.try_borrow_data()?)?;


        let total_emission = farm_main_data.total_coin_emission;
        msg!("total emission: {}", total_emission);
        let total_alloc = farm_main_data.total_alloc_point;


        let alloc = farm_state_data.alloc_point;
        msg!(" alloc: {}", alloc);
        
        let tvl = farm_state_data.total_value_locked;
        

        let user_stake = farm_user_data.staked_amount;

        msg!("total stake user: {}", user_stake);
        
        let alloc_ratio = alloc/total_alloc;
        msg!("alloc ratio {}", alloc_ratio);

        let farm_token_reward = total_emission ;

        msg!("farm reward {}", farm_token_reward);
        // change
        let user_reward = (farm_token_reward * user_stake * alloc )/ total_alloc / tvl ;
        msg!("user reward: ******************    {}", user_reward);



        let instruction = instruction::farm_harvest (
            user_account.key,
            user_info_account.key,
            farm_state_account.key,
            user_reward_account.key,
            pda_main_account.key,
            reward_mint.key,
            _spl_token_id.key,
            farm_program_id.key,
            amount,
        );
        msg!("berfore invoke");

        invoke(&instruction, accounts)?;

    } else {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    msg!("AmmInstruction::Harvest complete");
    Ok(())
}

/*

main:    total emission
        total alloc

state:  alloc
        TVL

user:  staked amount
        start time
    
        current time


fee ???????
*/