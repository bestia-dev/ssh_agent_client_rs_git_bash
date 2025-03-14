//! Error types
use thiserror::Error;

/// A Result variant with this module's `Error` as its error type
pub type Result<T> = std::result::Result<T, Error>;

/// This enum represents the different Errors that might be returned
/// by this crate.
#[derive(Error, Debug)]
pub enum Error {
    /// There was a failure connecting to git-bash ssh-agent
    #[error("Connection to git-bash ssh-agent error: {0}")]
    GitBashErrorMessage(String),
    /// There was an io::Error communicating with the agent
    #[error("An error occurred communicating with the agent")]
    AgentCommunication(#[from] std::io::Error),
    #[error(transparent)]
    SshAgentClientRs(#[from] ssh_agent_client_rs::Error),
}
