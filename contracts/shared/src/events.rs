//! # Event Definitions for AgroChain Contracts
//! 
//! This module defines all event types emitted by AgroChain smart contracts.

use soroban_sdk::{contractevent, Address, Bytes, Symbol, String, Vec, u64, u128, u32, u8};

/// Product registered event
#[contractevent]
pub struct ProductRegistered {
    pub product_id: Bytes,
    pub farmer: Address,
    pub product_name: String,
    pub category: u32,
    pub origin_lat: f64,
    pub origin_lon: f64,
    pub timestamp: u64,
}

/// Product status updated event
#[contractevent]
pub struct ProductStatusUpdated {
    pub product_id: Bytes,
    pub old_status: u32,
    pub new_status: u32,
    pub updated_by: Address,
    pub timestamp: u64,
}

/// Quality check completed event
#[contractevent]
pub struct QualityCheckCompleted {
    pub product_id: Bytes,
    pub quality_score: u8,
    pub grade: u32,
    pub inspector: Address,
    pub check_type: String,
    pub timestamp: u64,
}

/// Certificate issued event
#[contractevent]
pub struct CertificateIssued {
    pub certificate_id: Bytes,
    pub product_id: Option<Bytes>,
    pub cert_type: u32,
    pub issuer: Address,
    pub expires_at: Option<u64>,
    pub timestamp: u64,
}

/// Certificate verified event
#[contractevent]
pub struct CertificateVerified {
    pub certificate_id: Bytes,
    pub verified_by: Address,
    pub timestamp: u64,
}

/// Shipment created event
#[contractevent]
pub struct ShipmentCreated {
    pub shipment_id: Bytes,
    pub product_id: Bytes,
    pub shipper: Address,
    pub recipient: Address,
    pub origin_lat: f64,
    pub origin_lon: f64,
    pub dest_lat: f64,
    pub dest_lon: f64,
    pub timestamp: u64,
}

/// Shipment status updated event
#[contractevent]
pub struct ShipmentStatusUpdated {
    pub shipment_id: Bytes,
    pub old_status: u32,
    pub new_status: u32,
    pub updated_by: Address,
    pub timestamp: u64,
}

/// Temperature reading recorded event
#[contractevent]
pub struct TemperatureRecorded {
    pub shipment_id: Bytes,
    pub temperature: f32,
    pub sensor_id: Bytes,
    pub location_lat: f64,
    pub location_lon: f64,
    pub timestamp: u64,
}

/// Temperature violation detected event
#[contractevent]
pub struct TemperatureViolation {
    pub shipment_id: Bytes,
    pub temperature: f32,
    pub threshold: f32,
    pub duration: u64,
    pub timestamp: u64,
}

/// Payment processed event
#[contractevent]
pub struct PaymentProcessed {
    pub payment_id: Bytes,
    pub payer: Address,
    pub payee: Address,
    pub amount: u128,
    pub currency: Symbol,
    pub purpose: String,
    pub timestamp: u64,
}

/// Payment completed event
#[contractevent]
pub struct PaymentCompleted {
    pub payment_id: Bytes,
    pub transaction_hash: Bytes,
    pub timestamp: u64,
}

/// User registered event
#[contractevent]
pub struct UserRegistered {
    pub user_address: Address,
    pub user_type: u32,
    pub name: String,
    pub email: String,
    pub timestamp: u64,
}

/// User verified event
#[contractevent]
pub struct UserVerified {
    pub user_address: Address,
    pub verified_by: Address,
    pub timestamp: u64,
}

/// Alert created event
#[contractevent]
pub struct AlertCreated {
    pub alert_id: Bytes,
    pub alert_type: u32,
    pub severity: u32,
    pub product_id: Option<Bytes>,
    pub shipment_id: Option<Bytes>,
    pub message: String,
    pub source: Address,
    pub timestamp: u64,
}

/// Alert resolved event
#[contractevent]
pub struct AlertResolved {
    pub alert_id: Bytes,
    pub resolved_by: Address,
    pub resolution: String,
    pub timestamp: u64,
}

/// Sensor data recorded event
#[contractevent]
pub struct SensorDataRecorded {
    pub device_id: Bytes,
    pub sensor_type: String,
    pub value: f32,
    pub unit: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub timestamp: u64,
}

/// Batch created event
#[contractevent]
pub struct BatchCreated {
    pub batch_id: Bytes,
    pub product_ids: Vec<Bytes>,
    pub created_by: Address,
    pub timestamp: u64,
}

/// Batch updated event
#[contractevent]
pub struct BatchUpdated {
    pub batch_id: Bytes,
    pub added_products: Vec<Bytes>,
    pub removed_products: Vec<Bytes>,
    pub updated_by: Address,
    pub timestamp: u64,
}

/// Recall initiated event
#[contractevent]
pub struct RecallInitiated {
    pub product_id: Bytes,
    pub batch_id: Option<Bytes>,
    pub reason: String,
    pub initiated_by: Address,
    pub timestamp: u64,
}

/// Configuration updated event
#[contractevent]
pub struct ConfigurationUpdated {
    pub parameter: String,
    pub old_value: Bytes,
    pub new_value: Bytes,
    pub updated_by: Address,
    pub timestamp: u64,
}

/// Contract paused event
#[contractevent]
pub struct ContractPaused {
    pub paused_by: Address,
    pub reason: String,
    pub timestamp: u64,
}

/// Contract unpaused event
#[contractevent]
pub struct ContractUnpaused {
    pub unpaused_by: Address,
    pub reason: String,
    pub timestamp: u64,
}

/// Emergency brake activated event
#[contractevent]
pub struct EmergencyBrakeActivated {
    pub activated_by: Address,
    pub reason: String,
    pub timestamp: u64,
}

/// Emergency brake deactivated event
#[contractevent]
pub struct EmergencyBrakeDeactivated {
    pub deactivated_by: Address,
    pub reason: String,
    pub timestamp: u64,
}

/// Proposal created event
#[contractevent]
pub struct ProposalCreated {
    pub proposal_id: Bytes,
    pub proposer: Address,
    pub proposal_type: u32,
    pub title: String,
    pub voting_deadline: u64,
    pub timestamp: u64,
}

/// Vote cast event
#[contractevent]
pub struct VoteCast {
    pub proposal_id: Bytes,
    pub voter: Address,
    pub vote_choice: u32,
    pub voting_power: u32,
    pub timestamp: u64,
}

/// Proposal executed event
#[contractevent]
pub struct ProposalExecuted {
    pub proposal_id: Bytes,
    pub executed_by: Address,
    pub result: bool,
    pub timestamp: u64,
}

/// Ownership transferred event
#[contractevent]
pub struct OwnershipTransferred {
    pub product_id: Bytes,
    pub previous_owner: Address,
    pub new_owner: Address,
    pub timestamp: u64,
}

/// Tracking event recorded
#[contractevent]
pub struct TrackingEventRecorded {
    pub shipment_id: Bytes,
    pub event_type: String,
    pub location_lat: f64,
    pub location_lon: f64,
    pub description: String,
    pub recorded_by: Address,
    pub timestamp: u64,
}

/// Compliance check event
#[contractevent]
pub struct ComplianceCheck {
    pub entity_id: Bytes,
    pub entity_type: String,
    pub compliance_type: String,
    pub result: bool,
    pub checked_by: Address,
    pub timestamp: u64,
}

/// Audit initiated event
#[contractevent]
pub struct AuditInitiated {
    pub audit_target: Bytes,
    pub audit_type: String,
    pub initiated_by: Address,
    pub timestamp: u64,
}

/// Audit completed event
#[contractevent]
pub struct AuditCompleted {
    pub audit_id: Bytes,
    pub audit_target: Bytes,
    pub result: bool,
    pub findings: String,
    pub completed_by: Address,
    pub timestamp: u64,
}

/// Fee collected event
#[contractevent]
pub struct FeeCollected {
    pub from: Address,
    pub amount: u128,
    pub fee_type: String,
    pub timestamp: u64,
}

/// Reward distributed event
#[contractevent]
pub struct RewardDistributed {
    pub to: Address,
    pub amount: u128,
    pub reward_type: String,
    pub reason: String,
    pub timestamp: u64,
}

/// Stake deposited event
#[contractevent]
pub struct StakeDeposited {
    pub staker: Address,
    pub amount: u128,
    pub lock_period: u64,
    pub timestamp: u64,
}

/// Stake withdrawn event
#[contractevent]
pub struct StakeWithdrawn {
    pub staker: Address,
    pub amount: u128,
    pub rewards: u128,
    pub timestamp: u64,
}

/// slashing event
#[contractevent]
pub struct SlashingEvent {
    pub slashed_address: Address,
    pub amount: u128,
    pub reason: String,
    pub slashed_by: Address,
    pub timestamp: u64,
}

/// Upgrade initiated event
#[contractevent]
pub struct UpgradeInitiated {
    pub new_contract_address: Address,
    pub initiated_by: Address,
    pub timestamp: u64,
}

/// Migration event
#[contractevent]
pub struct MigrationEvent {
    pub from_contract: Address,
    pub to_contract: Address,
    pub data_migrated: Bytes,
    pub timestamp: u64,
}
