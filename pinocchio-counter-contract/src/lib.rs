#![no_std]
 
use pinocchio::{
    account_info::AccountInfo, 
    entrypoint, 
    nostd_panic_handler, 
    program_error::ProgramError,
    pubkey::Pubkey, 
    ProgramResult
};
 
entrypoint!(process_instruction);
nostd_panic_handler!();
 
pub mod instructions;
pub use instructions::*;

/// 22222222222222222222222222222222222222222222
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
];
 
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult { match instruction_data.split_first() {
        Some((&d, rest)) if d == *Initialize::DISCRIMINATOR => {
            // Pass both accounts and the rest of instruction_data
            let ix = Initialize::try_from((accounts, rest))?;
            ix.process()
        }

        Some((&d, rest)) if d == *Increment::DISCRIMINATOR => {
            let ix = Increment::try_from((accounts))?;
            ix.process()
        }

        Some((&d, rest)) if d == *Decrement::DISCRIMINATOR => {
            let ix = Decrement::try_from((accounts))?;
            ix.process()
        }

        _ => Err(ProgramError::InvalidInstructionData),
    }
}