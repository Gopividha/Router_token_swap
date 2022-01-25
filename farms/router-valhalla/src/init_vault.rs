use {
    crate::{
        utils::init_token_account,
        state::Vault,
    },
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg,
        system_instruction::create_account,
        program::invoke,
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
        pubkey::Pubkey,
        sysvar::rent::Rent,
    },
    farm::{
        instruction,
        state::{FarmMain,FarmState,FarmUserState},
    },
};


pub fn init_vault(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Processing Instruction : initVault");


    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    if let [
        admin_account,
        farm_state_account,
        pda_vault_account,
        vault_state_account,

        reward_token_account,
        lpmint_token_account,
        token_a_vault_account,
        token_b_vault_account,

        reward_mint,
        lp_mint,
        token_a_mint,
        token_b_mint,

        system_program_id,
        rent_program_id,
        token_program_id,
        farm_program_id,
        vault_program_id,

        ] = accounts
    {
        if *vault_program_id.owner != *admin_account.key{
            return Err(ProgramError::IllegalOwner);
        } 

        if !admin_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        //init vault state account
        invoke(
            &create_account(
                admin_account.key, 
                vault_state_account.key, 
                Rent::default().
                minimum_balance(
                Vault::LEN 
                ),
                Vault::LEN as u64, 
                vault_program_id.key,   
            ),
            &[
                admin_account.clone(),
                vault_state_account.clone(),
                system_program_id.clone(),
            ],
        )?;
        msg!("  vault state account ");



        let mut vault_info = Vault::unpack_unchecked(&vault_state_account.try_borrow_data()?)?;

        // check if vault account is already initialized
        if vault_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        // if escrow_info.vault_index > 0 {
        //     return Err(ProgramError::AccountAlreadyInitialized);
        // }

        // set the state for escrow account
        vault_info.is_initialized = true;
        vault_info.farm_state_account = *farm_state_account.key;
        vault_info.token_a_mint = *token_a_mint.key;
        vault_info.token_b_mint = *token_b_mint.key;
        vault_info.lp_token_custody_account = *lpmint_token_account.key;
        vault_info.reward_custody_account = *reward_token_account.key;
        vault_info.token_a_custody_account = *token_a_vault_account.key;
        vault_info.token_b_custody_account = *token_b_vault_account.key;
        vault_info.vault_index = 001 as u64;


        Vault::pack(vault_info, &mut vault_state_account.try_borrow_mut_data()?)?;

        msg!("  vault state packed  ");



        let farm_state_data = FarmState::unpack(&farm_state_account.try_borrow_data()?)?;
        let farm_ammid = farm_state_data.amm_id; 
        //pda with rewarder auth
        let pda_prefix  = "Val_VaultPDA";
        let pda_seed = &[
            pda_prefix.as_bytes(),
            farm_state_account.key.as_ref(),
        ];

        let (vault_pda, nonce) = Pubkey::find_program_address(pda_seed, vault_program_id.key);
        msg!("vault PDA {}", vault_pda);

        if vault_pda != *pda_vault_account.key{
            return Err(ProgramError::InvalidAccountData);
        }
        msg!("invoking init token account");
        msg!("  lpmint account ");
        init_token_account(
            admin_account,
            lpmint_token_account,
            lp_mint,
            pda_vault_account,
            rent_program_id,
            token_program_id,
            system_program_id,
        )?;

        msg!("  reward account ");
        init_token_account(
            admin_account,
            reward_token_account,
            reward_mint,
            pda_vault_account,
            rent_program_id,
            token_program_id,
            system_program_id,
        )?;


        msg!("  token A account ");
        init_token_account(
            admin_account,
            token_a_vault_account,
            token_a_mint,
            pda_vault_account,
            rent_program_id,
            token_program_id,
            system_program_id,
        )?;

        msg!("  token B account ");
        init_token_account(
            admin_account,
            token_b_vault_account,
            token_b_mint,
            pda_vault_account,
            rent_program_id,
            token_program_id,
            system_program_id,
        )?;
        
        // let pda_token_A_account_Ix = spl_associated_token_account::get_associated_token_address;(
        //     admin_account,
        //     pda_vault,
        //     token_a_mint,
        // )?;

        // invoke(
        //     pda_token_A_account_Ix,
        //     &[
        //         admin_account.clone(),
        //         pda_vault_account.clone(),
        //         spl_associated_token_account::id(),            ],
        // );

        msg!("done");     
    }
    Ok(())
}