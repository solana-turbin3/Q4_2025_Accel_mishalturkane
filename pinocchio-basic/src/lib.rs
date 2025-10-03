use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    ProgramResult,
};
use core::mem::size_of;

/// Instructions supported by this program
#[repr(C)]
pub enum Instruction {
    Increment,
    Decrement,
}

/// The account data layout
#[repr(C)]
pub struct CounterAccount {
    pub count: u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse instruction (0 = increment, 1 = decrement)
    let instruction = match instruction_data[0] {
        0 => Instruction::Increment,
        1 => Instruction::Decrement,
        _ => return Err(ProgramError::InvalidInstructionData),
    };

    // Expect first account = counter account
    let counter_ai = accounts
        .get(0)
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    // Derive PDA for "counter"
    let (expected_pda, _bump) = find_program_address(&[b"counter"], program_id);

    // Check that provided account is the PDA
    if *counter_ai.key != expected_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ownership check: PDA must belong to this program
    if counter_ai.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize counter account
    let mut data = counter_ai.try_borrow_mut_data()?;
    if data.len() < size_of::<CounterAccount>() {
        return Err(ProgramError::InvalidAccountData);
    }

    let counter: &mut CounterAccount =
        unsafe { &mut *(data.as_mut_ptr() as *mut CounterAccount) };

    // Apply instruction
    match instruction {
        Instruction::Increment => {
            counter.count = counter.count.saturating_add(1);
        }
        Instruction::Decrement => {
            counter.count = counter.count.saturating_sub(1);
        }
    }

    Ok(())
}
