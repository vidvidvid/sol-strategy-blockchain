use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// Define the program ID
solana_program::declare_id!("39VgcDSHAqL5xpFjHeEMwWj4j8rwuKTb1JNqiKFojW5g");

pub mod instruction;
pub mod processor;

// Program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::Processor::process(program_id, accounts, instruction_data)
}