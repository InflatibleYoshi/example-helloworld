use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Governance config
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StatPredictionConfiguration {
    /// The type of the vote threshold used for voting
    pub playersList Vec<String>,

    /// Boolean that denotes whether team won is a stat th 
    pub teamWon boolean,

    pub 

}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StatPredictionParameters {
    /// The type of the vote threshold used for voting

    pub 

}
