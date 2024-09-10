use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TradingInstruction {
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
    ExecuteTrade { amount: u64, is_buy: bool },
}

impl TradingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        TradingInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}