use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Governance config
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LOLStatPredictionConfiguration {
    /// The players in all teams. 
    pub playersList Vec<String>,

    /// Boolean that denotes whether team won is a stat used in computing score
    pub teamWon boolean,

    /// Boolean that denotes whether team won is a stat used in computing score
    pub 

}

impl

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StatPredictionParameters {
    /// The type of the vote threshold used for voting

    pub 

}