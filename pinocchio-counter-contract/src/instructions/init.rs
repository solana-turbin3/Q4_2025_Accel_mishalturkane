use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};
use core::mem::size_of;

pub struct CounterAccount<'a> {
    pub owner: &'a AccountInfo,
    pub counter_account: &'a AccountInfo,
}
impl <'a> TryFrom<&'a [AccountInfo]> for CounterAccount<'a> {
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
        if !counter_account.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);

        }

        if counter_account.lamports().ne(&0){
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let (counter_key, _ ) = find_program_address(&[b"counter",owner.key().as_ref()],&crate::ID);

        if &counter_key != counter_account.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

    

         Ok(Self { owner, counter_account })
    }
}


pub struct CounterInstructionData{
    pub counter: u64,
}

impl <'a> TryFrom<&'a [u8]> for CounterInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let counter = u64::from_le_bytes(data.try_into().unwrap());


        // insctruction data check
        if counter.eq(&0){
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok(Self { counter })
    }
}


pub struct  Initialize<'a>{
    pub accounts: CounterAccount<'a>,
    pub data: CounterInstructionData,
}

impl <'a> TryFrom<(&'a [AccountInfo],&'a [u8])> for Initialize<'a> {
    type Error = ProgramError;

    fn try_from((accounts,data): (&'a [AccountInfo],&'a [u8])) -> Result<Self, Self::Error> {
        let accounts = CounterAccount::try_from(accounts)?;
        let data = CounterInstructionData::try_from(data)?;

         Ok(Self { accounts, data })
    }
}


impl <'a> Initialize<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

      pub fn process(&self) -> ProgramResult {
        let counter_account = self.accounts.counter_account;

        // Get mutable data reference
        let mut data = counter_account.try_borrow_mut_data()?;

        // Initialize to 0
        data[..8].copy_from_slice(&0u64.to_le_bytes());

        Ok(())
    }

}