use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};

pub struct DecrementAccount<'a> {
    pub owner: &'a AccountInfo,
    pub counter_account: &'a AccountInfo,
    pub bump : [u8; 1],
}


impl <'a> TryFrom<&'a [AccountInfo]> for DecrementAccount<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner ,counter_account, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        //account check
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        //owner check
        if !counter_account.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);

        }

        if counter_account.lamports().eq(&0){
            return Err(ProgramError::UninitializedAccount);
        }

        let (counter_key, bump ) = find_program_address(&[b"counter",owner.key().as_ref()],&crate::ID);

        if &counter_key != counter_account.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

    

         Ok(Self { owner, counter_account, bump: [bump] })
    }
}

pub struct Decrement<'a> {
    pub accounts: DecrementAccount<'a>,
}

impl <'a> TryFrom<&'a [AccountInfo]> for Decrement<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        Ok(Self { accounts: DecrementAccount::try_from(accounts)? })
    }
}


impl <'a> Decrement<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

      pub fn process(&self) -> ProgramResult {
        let counter_account = self.accounts.counter_account;

        // Get mutable data reference
        let mut data = counter_account.try_borrow_mut_data()?;

        // Read current counter value
        let mut counter = u64::from_le_bytes(data[..8].try_into().unwrap());

        // Decrement counter
        counter = counter.checked_sub(1).ok_or(ProgramError::InvalidInstructionData)?;

        // Write back updated counter value
        data[..8].copy_from_slice(&counter.to_le_bytes());

        Ok(())
    }

}