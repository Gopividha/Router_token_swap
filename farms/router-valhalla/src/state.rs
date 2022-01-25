
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vault {
    pub is_initialized: bool,
    pub farm_state_account: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub lp_token_custody_account:Pubkey,
    pub reward_custody_account:Pubkey,
    pub token_a_custody_account:Pubkey,
    pub token_b_custody_account:Pubkey,
    pub vault_index: u64,
}
impl Sealed for Vault {}
impl IsInitialized for Vault {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for Vault {
    const LEN: usize = 233;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Vault::LEN];
        let (
            is_initialized,
            farm_state_account,
            token_a_mint,
            token_b_mint,
            lp_token_custody_account,
            reward_custody_account,
            token_a_custody_account,
            token_b_custody_account,
            vault_index,
        ) = array_refs![src, 1, 32, 32, 32, 32, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(Vault {
            is_initialized,
            farm_state_account: Pubkey::new_from_array(*farm_state_account),
            token_a_mint: Pubkey::new_from_array(*token_a_mint),
            token_b_mint: Pubkey::new_from_array(*token_b_mint),
            lp_token_custody_account: Pubkey::new_from_array(*lp_token_custody_account),
            reward_custody_account: Pubkey::new_from_array(*reward_custody_account),
            token_a_custody_account: Pubkey::new_from_array(*token_a_custody_account),
            token_b_custody_account: Pubkey::new_from_array(*token_b_custody_account),
            vault_index: u64::from_le_bytes(*vault_index),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Vault::LEN];
        let (
            is_initialized_dst,
            farm_state_account_dst,
            token_a_mint_dst,
            token_b_mint_dst,
            lp_token_custody_account_dst,
            reward_custody_account_dst,
            token_a_custody_account_dst,
            token_b_custody_account_dst,
            vault_index_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 32, 32, 32, 32, 8];
        let Vault {
            is_initialized,
            farm_state_account,
            token_a_mint,
            token_b_mint,
            lp_token_custody_account,
            reward_custody_account,
            token_a_custody_account,
            token_b_custody_account,
            vault_index,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        farm_state_account_dst.copy_from_slice(farm_state_account.as_ref());
        token_a_mint_dst.copy_from_slice(token_a_mint.as_ref());
        token_b_mint_dst.copy_from_slice(token_b_mint.as_ref());
        lp_token_custody_account_dst.copy_from_slice(lp_token_custody_account.as_ref());
        reward_custody_account_dst.copy_from_slice(reward_custody_account.as_ref());
        token_a_custody_account_dst.copy_from_slice(token_a_custody_account.as_ref());
        token_b_custody_account_dst.copy_from_slice(token_b_custody_account.as_ref());
        *vault_index_dst = vault_index.to_le_bytes();
    }
}