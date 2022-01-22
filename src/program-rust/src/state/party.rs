use solana_program::clock::{Slot, UnixTimestamp};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};

use participant::Participant;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Party {
    /// Party Name
    pub name: String,

    pub participants: Vec<Participant>,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Participant {
    pub account_info: Pubkey,
}
