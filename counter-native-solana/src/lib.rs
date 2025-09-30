use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("rust counter solana contract");

    let account = next_account_info(&mut accounts.iter())?; //pda account
    let user = next_account_info(&mut accounts.iter())?;  //wallet signer


     // Signer check
    if !user.is_signer {
        msg!("User did not sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 2. Derive PDA and verify
    let (pda, _bump) = Pubkey::find_program_address(&[b"counter", user.key.as_ref()], program_id);

    if account.key != &pda {
        msg!("Provided counter account does not match derived PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    // Ownership check
    if account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

   

    // Deserialize counter data
    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;

    // Deserialize the instruction data
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    // Safe arithmetic operations using checked_add and checked_sub
    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data.count = counter_data
                .count
                .checked_add(value)
                .ok_or(ProgramError::InvalidInstructionData)?;
        }
        InstructionType::Decrement(value) => {
            counter_data.count = counter_data
                .count
                .checked_sub(value)
                .ok_or(ProgramError::InvalidInstructionData)?;
        }
    }

    // Serialize the updated counter data back to the account
    counter_data.serialize(&mut *account.data.borrow_mut())?;

    msg!("Counter {} time(s)!", counter_data.count);

    Ok(())
}