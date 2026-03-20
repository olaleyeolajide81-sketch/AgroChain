//! # Error Definitions for AgroChain Contracts
//! 
//! This module defines all error types used across AgroChain smart contracts.

use soroban_sdk::{contracterror, Address, Symbol};

/// Contract error types
#[contracterror]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    /// General errors
    Unauthorized = 1000,
    InvalidInput = 1001,
    NotFound = 1002,
    AlreadyExists = 1003,
    InsufficientBalance = 1004,
    InsufficientAllowance = 1005,
    Overflow = 1006,
    Underflow = 1007,
    DivisionByZero = 1008,
    
    /// State errors
    ContractPaused = 2000,
    ContractNotPaused = 2001,
    InvalidState = 2002,
    StateTransitionNotAllowed = 2003,
    
    /// Configuration errors
    InvalidConfiguration = 3000,
    ConfigurationLocked = 3001,
    InvalidParameter = 3002,
    
    /// Access control errors
    AccessDenied = 4000,
    InvalidAddress = 4001,
    AddressBlacklisted = 4002,
    NotRegistered = 4003,
    AlreadyRegistered = 4004,
    
    /// Product errors
    ProductNotFound = 5000,
    InvalidProductStatus = 5001,
    ProductAlreadyRegistered = 5002,
    ProductNotOwned = 5003,
    ProductExpired = 5004,
    ProductRecalled = 5005,
    InvalidBatchSize = 5006,
    
    /// Quality errors
    QualityCheckFailed = 6000,
    InvalidQualityScore = 6001,
    QualityThresholdNotMet = 6002,
    InspectionRequired = 6003,
    InvalidInspector = 6004,
    
    /// Certification errors
    CertificateNotFound = 7000,
    CertificateExpired = 7001,
    InvalidCertificate = 7002,
    CertificateAlreadyVerified = 7003,
    InvalidIssuer = 7004,
    
    /// Shipment errors
    ShipmentNotFound = 8000,
    InvalidShipmentStatus = 8001,
    ShipmentAlreadyDelivered = 8002,
    TemperatureViolation = 8003,
    DelayedShipment = 8004,
    InvalidRoute = 8005,
    
    /// Payment errors
    PaymentNotFound = 9000,
    InvalidPaymentAmount = 9001,
    PaymentFailed = 9002,
    PaymentAlreadyCompleted = 9003,
    InvalidCurrency = 9004,
    InsufficientFee = 9005,
    
    /// Governance errors
    ProposalNotFound = 10000,
    ProposalAlreadyExecuted = 10001,
    VotingPeriodEnded = 10002,
    AlreadyVoted = 10003,
    InvalidVote = 10004,
    QuorumNotMet = 10005,
    
    /// IoT errors
    SensorNotFound = 11000,
    InvalidSensorData = 11001,
    SensorOffline = 11002,
    DataQualityLow = 11003,
    
    /// Alert errors
    AlertNotFound = 12000,
    AlertAlreadyResolved = 12001,
    InvalidAlertType = 12002,
    
    /// System errors
    InternalError = 13000,
    NetworkError = 13001,
    TemporarilyUnavailable = 13002,
    RateLimitExceeded = 13003,
    
    /// Compliance errors
    ComplianceViolation = 14000,
    RegulatoryRequirement = 14001,
    AuditRequired = 14002,
    
    /// Emergency errors
    EmergencyBrakeActivated = 15000,
    EmergencyActionRequired = 15001,
    SystemShutdown = 15002,
}

/// Result type for contract operations
pub type ContractResult<T> = Result<T, ContractError>;

/// Error messages for debugging
impl ContractError {
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            ContractError::Unauthorized => "Unauthorized access",
            ContractError::InvalidInput => "Invalid input provided",
            ContractError::NotFound => "Resource not found",
            ContractError::AlreadyExists => "Resource already exists",
            ContractError::InsufficientBalance => "Insufficient balance",
            ContractError::InsufficientAllowance => "Insufficient allowance",
            ContractError::Overflow => "Arithmetic overflow",
            ContractError::Underflow => "Arithmetic underflow",
            ContractError::DivisionByZero => "Division by zero",
            
            ContractError::ContractPaused => "Contract is paused",
            ContractError::ContractNotPaused => "Contract is not paused",
            ContractError::InvalidState => "Invalid contract state",
            ContractError::StateTransitionNotAllowed => "State transition not allowed",
            
            ContractError::InvalidConfiguration => "Invalid configuration",
            ContractError::ConfigurationLocked => "Configuration is locked",
            ContractError::InvalidParameter => "Invalid parameter",
            
            ContractError::AccessDenied => "Access denied",
            ContractError::InvalidAddress => "Invalid address",
            ContractError::AddressBlacklisted => "Address is blacklisted",
            ContractError::NotRegistered => "Not registered",
            ContractError::AlreadyRegistered => "Already registered",
            
            ContractError::ProductNotFound => "Product not found",
            ContractError::InvalidProductStatus => "Invalid product status",
            ContractError::ProductAlreadyRegistered => "Product already registered",
            ContractError::ProductNotOwned => "Product not owned",
            ContractError::ProductExpired => "Product expired",
            ContractError::ProductRecalled => "Product recalled",
            ContractError::InvalidBatchSize => "Invalid batch size",
            
            ContractError::QualityCheckFailed => "Quality check failed",
            ContractError::InvalidQualityScore => "Invalid quality score",
            ContractError::QualityThresholdNotMet => "Quality threshold not met",
            ContractError::InspectionRequired => "Inspection required",
            ContractError::InvalidInspector => "Invalid inspector",
            
            ContractError::CertificateNotFound => "Certificate not found",
            ContractError::CertificateExpired => "Certificate expired",
            ContractError::InvalidCertificate => "Invalid certificate",
            ContractError::CertificateAlreadyVerified => "Certificate already verified",
            ContractError::InvalidIssuer => "Invalid issuer",
            
            ContractError::ShipmentNotFound => "Shipment not found",
            ContractError::InvalidShipmentStatus => "Invalid shipment status",
            ContractError::ShipmentAlreadyDelivered => "Shipment already delivered",
            ContractError::TemperatureViolation => "Temperature violation",
            ContractError::DelayedShipment => "Delayed shipment",
            ContractError::InvalidRoute => "Invalid route",
            
            ContractError::PaymentNotFound => "Payment not found",
            ContractError::InvalidPaymentAmount => "Invalid payment amount",
            ContractError::PaymentFailed => "Payment failed",
            ContractError::PaymentAlreadyCompleted => "Payment already completed",
            ContractError::InvalidCurrency => "Invalid currency",
            ContractError::InsufficientFee => "Insufficient fee",
            
            ContractError::ProposalNotFound => "Proposal not found",
            ContractError::ProposalAlreadyExecuted => "Proposal already executed",
            ContractError::VotingPeriodEnded => "Voting period ended",
            ContractError::AlreadyVoted => "Already voted",
            ContractError::InvalidVote => "Invalid vote",
            ContractError::QuorumNotMet => "Quorum not met",
            
            ContractError::SensorNotFound => "Sensor not found",
            ContractError::InvalidSensorData => "Invalid sensor data",
            ContractError::SensorOffline => "Sensor offline",
            ContractError::DataQualityLow => "Data quality low",
            
            ContractError::AlertNotFound => "Alert not found",
            ContractError::AlertAlreadyResolved => "Alert already resolved",
            ContractError::InvalidAlertType => "Invalid alert type",
            
            ContractError::InternalError => "Internal error",
            ContractError::NetworkError => "Network error",
            ContractError::TemporarilyUnavailable => "Temporarily unavailable",
            ContractError::RateLimitExceeded => "Rate limit exceeded",
            
            ContractError::ComplianceViolation => "Compliance violation",
            ContractError::RegulatoryRequirement => "Regulatory requirement",
            ContractError::AuditRequired => "Audit required",
            
            ContractError::EmergencyBrakeActivated => "Emergency brake activated",
            ContractError::EmergencyActionRequired => "Emergency action required",
            ContractError::SystemShutdown => "System shutdown",
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ContractError::Unauthorized => false,
            ContractError::InvalidInput => true,
            ContractError::NotFound => false,
            ContractError::AlreadyExists => false,
            ContractError::InsufficientBalance => true,
            ContractError::InsufficientAllowance => true,
            ContractError::Overflow => false,
            ContractError::Underflow => false,
            ContractError::DivisionByZero => false,
            
            ContractError::ContractPaused => true,
            ContractError::ContractNotPaused => false,
            ContractError::InvalidState => false,
            ContractError::StateTransitionNotAllowed => true,
            
            ContractError::InvalidConfiguration => true,
            ContractError::ConfigurationLocked => false,
            ContractError::InvalidParameter => true,
            
            ContractError::AccessDenied => false,
            ContractError::InvalidAddress => true,
            ContractError::AddressBlacklisted => false,
            ContractError::NotRegistered => true,
            ContractError::AlreadyRegistered => false,
            
            ContractError::ProductNotFound => false,
            ContractError::InvalidProductStatus => true,
            ContractError::ProductAlreadyRegistered => false,
            ContractError::ProductNotOwned => false,
            ContractError::ProductExpired => false,
            ContractError::ProductRecalled => false,
            ContractError::InvalidBatchSize => true,
            
            ContractError::QualityCheckFailed => true,
            ContractError::InvalidQualityScore => true,
            ContractError::QualityThresholdNotMet => true,
            ContractError::InspectionRequired => true,
            ContractError::InvalidInspector => true,
            
            ContractError::CertificateNotFound => false,
            ContractError::CertificateExpired => true,
            ContractError::InvalidCertificate => true,
            ContractError::CertificateAlreadyVerified => false,
            ContractError::InvalidIssuer => true,
            
            ContractError::ShipmentNotFound => false,
            ContractError::InvalidShipmentStatus => true,
            ContractError::ShipmentAlreadyDelivered => false,
            ContractError::TemperatureViolation => true,
            ContractError::DelayedShipment => true,
            ContractError::InvalidRoute => true,
            
            ContractError::PaymentNotFound => false,
            ContractError::InvalidPaymentAmount => true,
            ContractError::PaymentFailed => true,
            ContractError::PaymentAlreadyCompleted => false,
            ContractError::InvalidCurrency => true,
            ContractError::InsufficientFee => true,
            
            ContractError::ProposalNotFound => false,
            ContractError::ProposalAlreadyExecuted => false,
            ContractError::VotingPeriodEnded => false,
            ContractError::AlreadyVoted => false,
            ContractError::InvalidVote => true,
            ContractError::QuorumNotMet => false,
            
            ContractError::SensorNotFound => false,
            ContractError::InvalidSensorData => true,
            ContractError::SensorOffline => true,
            ContractError::DataQualityLow => true,
            
            ContractError::AlertNotFound => false,
            ContractError::AlertAlreadyResolved => false,
            ContractError::InvalidAlertType => true,
            
            ContractError::InternalError => false,
            ContractError::NetworkError => true,
            ContractError::TemporarilyUnavailable => true,
            ContractError::RateLimitExceeded => true,
            
            ContractError::ComplianceViolation => false,
            ContractError::RegulatoryRequirement => false,
            ContractError::AuditRequired => true,
            
            ContractError::EmergencyBrakeActivated => false,
            ContractError::EmergencyActionRequired => false,
            ContractError::SystemShutdown => false,
        }
    }
    
    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ContractError::Unauthorized => ErrorSeverity::High,
            ContractError::InvalidInput => ErrorSeverity::Low,
            ContractError::NotFound => ErrorSeverity::Medium,
            ContractError::AlreadyExists => ErrorSeverity::Medium,
            ContractError::InsufficientBalance => ErrorSeverity::Medium,
            ContractError::InsufficientAllowance => ErrorSeverity::Medium,
            ContractError::Overflow => ErrorSeverity::Critical,
            ContractError::Underflow => ErrorSeverity::Critical,
            ContractError::DivisionByZero => ErrorSeverity::Critical,
            
            ContractError::ContractPaused => ErrorSeverity::Medium,
            ContractError::ContractNotPaused => ErrorSeverity::Low,
            ContractError::InvalidState => ErrorSeverity::High,
            ContractError::StateTransitionNotAllowed => ErrorSeverity::Medium,
            
            ContractError::InvalidConfiguration => ErrorSeverity::High,
            ContractError::ConfigurationLocked => ErrorSeverity::Medium,
            ContractError::InvalidParameter => ErrorSeverity::Low,
            
            ContractError::AccessDenied => ErrorSeverity::High,
            ContractError::InvalidAddress => ErrorSeverity::Low,
            ContractError::AddressBlacklisted => ErrorSeverity::High,
            ContractError::NotRegistered => ErrorSeverity::Medium,
            ContractError::AlreadyRegistered => ErrorSeverity::Medium,
            
            ContractError::ProductNotFound => ErrorSeverity::Medium,
            ContractError::InvalidProductStatus => ErrorSeverity::Medium,
            ContractError::ProductAlreadyRegistered => ErrorSeverity::Medium,
            ContractError::ProductNotOwned => ErrorSeverity::High,
            ContractError::ProductExpired => ErrorSeverity::Medium,
            ContractError::ProductRecalled => ErrorSeverity::High,
            ContractError::InvalidBatchSize => ErrorSeverity::Low,
            
            ContractError::QualityCheckFailed => ErrorSeverity::High,
            ContractError::InvalidQualityScore => ErrorSeverity::Low,
            ContractError::QualityThresholdNotMet => ErrorSeverity::High,
            ContractError::InspectionRequired => ErrorSeverity::Medium,
            ContractError::InvalidInspector => ErrorSeverity::Medium,
            
            ContractError::CertificateNotFound => ErrorSeverity::Medium,
            ContractError::CertificateExpired => ErrorSeverity::Medium,
            ContractError::InvalidCertificate => ErrorSeverity::High,
            ContractError::CertificateAlreadyVerified => ErrorSeverity::Low,
            ContractError::InvalidIssuer => ErrorSeverity::High,
            
            ContractError::ShipmentNotFound => ErrorSeverity::Medium,
            ContractError::InvalidShipmentStatus => ErrorSeverity::Medium,
            ContractError::ShipmentAlreadyDelivered => ErrorSeverity::Low,
            ContractError::TemperatureViolation => ErrorSeverity::High,
            ContractError::DelayedShipment => ErrorSeverity::Medium,
            ContractError::InvalidRoute => ErrorSeverity::Medium,
            
            ContractError::PaymentNotFound => ErrorSeverity::Medium,
            ContractError::InvalidPaymentAmount => ErrorSeverity::Low,
            ContractError::PaymentFailed => ErrorSeverity::High,
            ContractError::PaymentAlreadyCompleted => ErrorSeverity::Low,
            ContractError::InvalidCurrency => ErrorSeverity::Medium,
            ContractError::InsufficientFee => ErrorSeverity::Medium,
            
            ContractError::ProposalNotFound => ErrorSeverity::Medium,
            ContractError::ProposalAlreadyExecuted => ErrorSeverity::Low,
            ContractError::VotingPeriodEnded => ErrorSeverity::Medium,
            ContractError::AlreadyVoted => ErrorSeverity::Low,
            ContractError::InvalidVote => ErrorSeverity::Low,
            ContractError::QuorumNotMet => ErrorSeverity::Medium,
            
            ContractError::SensorNotFound => ErrorSeverity::Medium,
            ContractError::InvalidSensorData => ErrorSeverity::Medium,
            ContractError::SensorOffline => ErrorSeverity::Medium,
            ContractError::DataQualityLow => ErrorSeverity::Low,
            
            ContractError::AlertNotFound => ErrorSeverity::Medium,
            ContractError::AlertAlreadyResolved => ErrorSeverity::Low,
            ContractError::InvalidAlertType => ErrorSeverity::Low,
            
            ContractError::InternalError => ErrorSeverity::Critical,
            ContractError::NetworkError => ErrorSeverity::Medium,
            ContractError::TemporarilyUnavailable => ErrorSeverity::Medium,
            ContractError::RateLimitExceeded => ErrorSeverity::Low,
            
            ContractError::ComplianceViolation => ErrorSeverity::High,
            ContractError::RegulatoryRequirement => ErrorSeverity::High,
            ContractError::AuditRequired => ErrorSeverity::Medium,
            
            ContractError::EmergencyBrakeActivated => ErrorSeverity::Critical,
            ContractError::EmergencyActionRequired => ErrorSeverity::Critical,
            ContractError::SystemShutdown => ErrorSeverity::Critical,
        }
    }
}

/// Error severity levels
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}
