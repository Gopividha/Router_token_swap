use {
    solana_program::{
        account_info::AccountInfo,
         entrypoint::ProgramResult,
        program, 
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar,
        sysvar::Sysvar,
        msg,
    },
};


pub fn init_token_account<'a, 'b>(
    funding_account: &'a AccountInfo<'b>,
    new_account: &'a AccountInfo<'b>,
    mint_account: &'a AccountInfo<'b>,
    owner_account: &'a AccountInfo<'b>,  //pda
    rent_program: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    system_program:&'a AccountInfo<'b>,
) -> ProgramResult {

    msg!("ini utils");
    if !new_account.data_is_empty() {
        return Ok(());
    }

    program::invoke(
        &system_instruction::create_account(
            funding_account.key, 
            new_account.key, 
            Rent::default().
            minimum_balance(
            spl_token::state::Account::LEN 
            ),
            spl_token::state::Account::LEN as u64, 
            token_program.key,   
        ),
        &[
            funding_account.clone(),
            new_account.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;
    msg!("token acc created");

    program::invoke(
        &spl_token::instruction::initialize_account(
            &spl_token::id(),
            new_account.key,
            mint_account.key,
            owner_account.key,
        )?,
        &[
            new_account.clone(),
            mint_account.clone(),
            owner_account.clone(),
            rent_program.clone(),
        ],
    )?;
    msg!("new account:  {}",new_account.key);

    Ok(())
}
