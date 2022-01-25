// compounding
use {
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg,
        program::invoke,
        program_error::ProgramError,
        program_pack::Pack,
        sysvar::{rent::Rent, Sysvar, clock::Clock},

    },
    farm::{
        instruction,
        state::{FarmMain,FarmState,FarmUserState},
    },
};

pub fn val_crank1(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Processing AmmInstruction::Cranking");
    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        user_account,
        user_state_account,
        farm_main_val_account,
        farm_state_account,
        user_reward_account,
        pda_main_account,
        reward_mint,
        // token_a_mint,
        // token_b_mint,
        _spl_token_id,
        farm_program_id,
        ] = accounts
    {
        // let amount:u64=1200;
        // msg!("enter");


        msg!("accounts ");
        let farm_user_data = FarmUserState::unpack(&user_state_account.try_borrow_data()?)?;
        //let farm_state_account = farm_user_data.farm_state;

        let farm_main_data = FarmMain::unpack(&farm_main_val_account.try_borrow_data()?)?;
        let farm_state_data = FarmState::unpack(&farm_state_account.try_borrow_data()?)?;

msg!("unpacked............");
        let total_emission = farm_main_data.total_coin_emission;
        msg!("total emission: {}", total_emission);
        let total_alloc = farm_main_data.total_alloc_point;


        let alloc = farm_state_data.alloc_point;
        msg!(" alloc: {}", alloc);
        

        
        let tvl = farm_state_data.total_value_locked;

        

        let user_stake = farm_user_data.staked_amount;

        msg!("total stake user: {}", user_stake);
        
        // let alloc_ratio = alloc/total_alloc;
        // msg!("alloc ratio {}", alloc_ratio);

        // let farm_token_reward = total_emission ;

        // msg!("farm reward {}", farm_token_reward);
        // change

        let user_reward_pre_day = (total_emission * user_stake * alloc )/ total_alloc / tvl ;
        msg!("user reward: ******************    {}", user_reward_pre_day);


        let system_clock=Clock::get()?;

        let mut last_crank = farm_user_data.last_crank_time;
        msg!("last_crank_time{}",last_crank);
        msg!("system_clock {}",system_clock.unix_timestamp);

        if last_crank == 0 {
            last_crank = farm_user_data.start_time;
        }

        let interval =system_clock.unix_timestamp as u64 - last_crank;
        let no_of_days=interval/86400;

        msg!("no_of_days {}",no_of_days);


        let instruction = instruction::farm_harvest(
            user_account.key,
            user_state_account.key,
            farm_state_account.key,
            user_reward_account.key,
            pda_main_account.key,
            reward_mint.key,
            _spl_token_id.key,
            farm_program_id.key,
            user_reward_pre_day,
        );
        msg!("berfore invoke");

        invoke(&instruction, accounts)?;

    }
    
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    msg!("AmmInstruction::cranking- Harvest complete");
    Ok(())
    
}


///////////


/*


Tvl : 302 LP
Total_emission || total_allloc || alloc_point_1st || alloc_point_2nd
	100000		100000		    50000		        40




Per LP :  ( 60/100*1000 ) / 200

Per user :  per LP * staked_amount


*/