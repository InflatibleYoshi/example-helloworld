use solana_program::clock::{Slot, UnixTimestamp};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};

/// Governance config
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct DraftConfiguration {
    /// The type of the vote threshold used for voting
    pub vote_threshold_percentage: VoteThresholdPercentage,

    pub draft_participant_minimum: u32,

    pub draft_participant_maximum: u32,

    pub draft_format: DraftType,

    pub draft_
}