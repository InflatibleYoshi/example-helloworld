//! Error types

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the Governance program
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]

pub enum GovernanceError {
    /// Invalid instruction passed to program
    #[error("Invalid instruction passed to program")]
    InvalidInstruction = 500, // Start Governance custom errors from 500 to avoid conflicts with programs invoked via CPI
}
