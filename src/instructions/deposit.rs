use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address};



pub struct DepositAccounts<'a>{
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a>{
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        
        if !owner.is_signer(){
            return Err(ProgramError::InvalidAccountOwner);
        }
        
        if vault.is_owned_by(&pinocchio_system::ID){
            return Err(ProgramError::InvalidAccountOwner);
        }
        
        if vault.lamports().ne(&0){
            return Err(ProgramError::InvalidAccountData);
        }
        
        let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        if vault.key().ne(&vault_key){
            return Err(ProgramError::InvalidAccountOwner)
        }
        
        Ok(Self { owner, vault })
    }
}