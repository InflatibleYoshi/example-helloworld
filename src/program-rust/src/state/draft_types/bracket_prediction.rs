
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Governance config
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BracketPredictionConfiguration {
   
    /// The type of the vote threshold used for voting
    pub teamsList Vec<String, String>,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LinkedBracketNode {

    pub winningTeam String,

    pub losingTeam String,

    pub previousMatchUpper BracketNode,

    pub previousMatchLower BracketNode,

}