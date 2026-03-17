//! # AgroChain Shared Library
//! 
//! This library provides shared types, errors, events, and utilities
//! used across all AgroChain smart contracts.

pub mod types;
pub mod errors;
pub mod events;
pub mod utils;
pub mod constants;

// Re-export commonly used items
pub use types::*;
pub use errors::*;
pub use events::*;
pub use constants::*;

use soroban_sdk::{contracttype, Address, Bytes, Env, Symbol};

/// Contract state structure for shared functionality
#[contracttype]
pub struct ContractState {
    /// Contract administrator
    pub admin: Address,
    /// Contract paused status
    pub paused: bool,
    /// Configuration settings
    pub config: Configuration,
}

/// Configuration structure for contracts
#[contracttype]
pub struct Configuration {
    /// Minimum stake amount
    pub min_stake: u128,
    /// Maximum batch size
    pub max_batch_size: u32,
    /// Quality threshold
    pub quality_threshold: u8,
    /// Fee percentage (basis points)
    pub fee_percentage: u32,
}

impl ContractState {
    /// Create new contract state
    pub fn new(admin: Address) -> Self {
        Self {
            admin,
            paused: false,
            config: Configuration {
                min_stake: 1000000, // 0.1 XLM
                max_batch_size: 1000,
                quality_threshold: 70,
                fee_percentage: 100, // 1%
            },
        }
    }
}

impl Configuration {
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.min_stake == 0 {
            return Err(ContractError::InvalidConfiguration);
        }
        if self.max_batch_size == 0 {
            return Err(ContractError::InvalidConfiguration);
        }
        if self.quality_threshold == 0 || self.quality_threshold > 100 {
            return Err(ContractError::InvalidConfiguration);
        }
        if self.fee_percentage > 10000 { // 100%
            return Err(ContractError::InvalidConfiguration);
        }
        Ok(())
    }
}

/// Utility functions for contract operations
pub mod utils {
    use super::*;
    use soroban_sdk::{Env, Address, Bytes};

    /// Calculate fee amount based on percentage
    pub fn calculate_fee(amount: u128, percentage: u32) -> u128 {
        (amount * u128::from(percentage)) / 10000
    }

    /// Validate address format
    pub fn validate_address(address: &Address) -> bool {
        !address.is_zero()
    }

    /// Generate unique ID using timestamp and random bytes
    pub fn generate_unique_id(env: &Env, prefix: &Symbol) -> Bytes {
        let timestamp = env.ledger().timestamp();
        let random_bytes = env.prng().gen::<[u8; 16]>();
        let mut id_bytes = prefix.to_bytes();
        id_bytes.extend_from_slice(&timestamp.to_be_bytes());
        id_bytes.extend_from_slice(&random_bytes);
        Bytes::from_slice(&id_bytes)
    }

    /// Check if contract is paused
    pub fn is_paused(env: &Env) -> bool {
        let state: ContractState = env.storage().instance().get(&Symbol::new(&env, "STATE")).unwrap();
        state.paused
    }
}

/// Constants used across contracts
pub mod constants {
    use soroban_sdk::Symbol;

    /// Storage keys
    pub const STATE_KEY: Symbol = Symbol::new(&"STATE");
    pub const ADMIN_KEY: Symbol = Symbol::new(&"ADMIN");
    pub const CONFIG_KEY: Symbol = Symbol::new(&"CONFIG");

    /// Quality thresholds
    pub const MIN_QUALITY_SCORE: u8 = 0;
    pub const MAX_QUALITY_SCORE: u8 = 100;
    pub const DEFAULT_QUALITY_THRESHOLD: u8 = 70;

    /// Fee constants
    pub const MIN_FEE_PERCENTAGE: u32 = 0;
    pub const MAX_FEE_PERCENTAGE: u32 = 1000; // 10%
    pub const DEFAULT_FEE_PERCENTAGE: u32 = 100; // 1%

    /// Batch size limits
    pub const MIN_BATCH_SIZE: u32 = 1;
    pub const MAX_BATCH_SIZE: u32 = 10000;
    pub const DEFAULT_BATCH_SIZE: u32 = 100;

    /// Stake amounts (in stroops)
    pub const MIN_STAKE_AMOUNT: u128 = 1000000; // 0.1 XLM
    pub const DEFAULT_STAKE_AMOUNT: u128 = 10000000; // 1 XLM
    pub const MAX_STAKE_AMOUNT: u128 = 10000000000; // 1000 XLM
}
