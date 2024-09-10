use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use crate::instruction::TradingInstruction;

pub struct Processor;

impl Processor {
    pub fn process(_program_id: &Pubkey, _accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = TradingInstruction::unpack(instruction_data)?;

        match instruction {
            TradingInstruction::Deposit { amount } => {
                msg!("Instruction: Deposit {}", amount);
                // Add deposit logic here
            }
            TradingInstruction::Withdraw { amount } => {
                msg!("Instruction: Withdraw {}", amount);
                // Add withdraw logic here
            }
            TradingInstruction::ExecuteTrade { amount, is_buy } => {
                msg!("Instruction: Execute Trade, Amount: {}, Is Buy: {}", amount, is_buy);
                // Add trade execution logic here
            }
        }

        Ok(())
    }
}