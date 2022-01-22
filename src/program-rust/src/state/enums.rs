use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Defines all Governance accounts types
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum DraftState {
    OpenParty,

    ClosedParty,

    DraftInProgress,

    PostDraft,

    Payout,
}

/// Defines all Governance accounts types
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum DraftType {
    StatPrediction,

    BracketPrediction,

    TeamManager,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum PartyState {
    Uninitialized,

    Active,

    Abandoned,
}

impl Default for PartyState {
    fn default() -> Self {
        PartyState::Uninitialized
    }
}
