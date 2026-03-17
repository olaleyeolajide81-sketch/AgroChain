//! # Type Definitions for AgroChain Contracts
//! 
//! This module defines all the data types used across AgroChain smart contracts.

use soroban_sdk::{
    contracttype, Address, Bytes, Symbol, String, Vec, Map, u64, u128, i128, u32, i32, u8, bool, Env,
};

/// Product category enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductCategory {
    Vegetables,
    Fruits,
    Grains,
    Dairy,
    Meat,
    Seafood,
    Herbs,
    Spices,
    Other,
}

/// Product status enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductStatus {
    Registered,
    Growing,
    Harvested,
    Processing,
    Packaged,
    Shipped,
    Delivered,
    Recalled,
    Expired,
}

/// Quality grade enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QualityGrade {
    A,      // Premium quality
    B,      // Good quality
    C,      // Standard quality
    D,      // Below standard
    F,       // Poor quality
}

/// Certification type enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CertificationType {
    Organic,
    FairTrade,
    NonGMO,
    GAP,      // Good Agricultural Practices
    HACCP,    // Hazard Analysis Critical Control Points
    ISO22000,  // Food Safety Management
    Other,
}

/// Location information
#[contracttype]
#[derive(Clone, Debug)]
pub struct Location {
    /// Latitude coordinate
    pub latitude: f64,
    /// Longitude coordinate
    pub longitude: f64,
    /// Physical address
    pub address: String,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Region or state
    pub region: String,
}

/// Product information structure
#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    /// Unique product identifier
    pub id: Bytes,
    /// Product name
    pub name: String,
    /// Product description
    pub description: String,
    /// Product category
    pub category: ProductCategory,
    /// Current status
    pub status: ProductStatus,
    /// Farmer/producer address
    pub farmer: Address,
    /// Origin location
    pub origin: Location,
    /// Current owner
    pub owner: Address,
    /// Registration timestamp
    pub created_at: u64,
    /// Last updated timestamp
    pub updated_at: u64,
    /// Metadata hash (for additional data)
    pub metadata_hash: Bytes,
    /// Quality grade
    pub quality_grade: Option<QualityGrade>,
    /// Certifications
    pub certifications: Vec<Certificate>,
    /// Batch identifier
    pub batch_id: Option<Bytes>,
    /// Expiration timestamp
    pub expires_at: Option<u64>,
}

/// Certificate information
#[contracttype]
#[derive(Clone, Debug)]
pub struct Certificate {
    /// Certificate identifier
    pub id: Bytes,
    /// Certificate type
    pub cert_type: CertificationType,
    /// Issuing authority
    pub issuer: Address,
    /// Issue timestamp
    pub issued_at: u64,
    /// Expiration timestamp
    pub expires_at: Option<u64>,
    /// Certificate data hash
    pub data_hash: Bytes,
    /// Verification status
    pub verified: bool,
}

/// Shipment information
#[contracttype]
#[derive(Clone, Debug)]
pub struct Shipment {
    /// Unique shipment identifier
    pub id: Bytes,
    /// Product identifier
    pub product_id: Bytes,
    /// Shipper address
    pub shipper: Address,
    /// Recipient address
    pub recipient: Address,
    /// Origin location
    pub origin: Location,
    /// Destination location
    pub destination: Location,
    /// Shipment timestamp
    pub shipped_at: u64,
    /// Expected delivery timestamp
    pub expected_delivery: Option<u64>,
    /// Actual delivery timestamp
    pub delivered_at: Option<u64>,
    /// Current status
    pub status: ShipmentStatus,
    /// Temperature logs during transit
    pub temperature_logs: Vec<TemperatureReading>,
    /// Quality checks
    pub quality_checks: Vec<QualityCheck>,
    /// Tracking events
    pub tracking_events: Vec<TrackingEvent>,
}

/// Shipment status enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShipmentStatus {
    Preparing,
    InTransit,
    Delayed,
    Delivered,
    Lost,
    Damaged,
    Returned,
}

/// Temperature reading for cold chain monitoring
#[contracttype]
#[derive(Clone, Debug)]
pub struct TemperatureReading {
    /// Reading timestamp
    pub timestamp: u64,
    /// Temperature in Celsius
    pub temperature: f32,
    /// Sensor location
    pub location: Location,
    /// Sensor identifier
    pub sensor_id: Bytes,
    /// Reading quality score
    pub quality_score: u8,
}

/// Quality check result
#[contracttype]
#[derive(Clone, Debug)]
pub struct QualityCheck {
    /// Check timestamp
    pub timestamp: u64,
    /// Quality score (0-100)
    pub score: u8,
    /// Grade assigned
    pub grade: QualityGrade,
    /// Inspector address
    pub inspector: Address,
    /// Check type
    pub check_type: String,
    /// Check result data hash
    pub data_hash: Bytes,
    /// Comments
    pub comments: String,
}

/// Tracking event
#[contracttype]
#[derive(Clone, Debug)]
pub struct TrackingEvent {
    /// Event timestamp
    pub timestamp: u64,
    /// Event location
    pub location: Location,
    /// Event type
    pub event_type: String,
    /// Event description
    pub description: String,
    /// Recorded by
    pub recorded_by: Address,
    /// Additional metadata
    pub metadata: Bytes,
}

/// Payment transaction
#[contracttype]
#[derive(Clone, Debug)]
pub struct Payment {
    /// Payment identifier
    pub id: Bytes,
    /// Related product ID
    pub product_id: Option<Bytes>,
    /// Related shipment ID
    pub shipment_id: Option<Bytes>,
    /// Payer address
    pub payer: Address,
    /// Payee address
    pub payee: Address,
    /// Amount in stroops
    pub amount: u128,
    /// Currency (XLM or token)
    pub currency: Symbol,
    /// Payment timestamp
    pub timestamp: u64,
    /// Payment status
    pub status: PaymentStatus,
    /// Transaction hash on Stellar
    pub transaction_hash: Bytes,
    /// Payment purpose
    pub purpose: String,
}

/// Payment status enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
    Disputed,
}

/// User profile information
#[contracttype]
#[derive(Clone, Debug)]
pub struct UserProfile {
    /// User address
    pub address: Address,
    /// User type
    pub user_type: UserType,
    /// Display name
    pub name: String,
    /// Contact email
    pub email: String,
    /// Profile verification status
    pub verified: bool,
    /// Registration timestamp
    pub registered_at: u64,
    /// Reputation score
    pub reputation: u32,
    /// Additional metadata
    pub metadata: Bytes,
}

/// User type enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserType {
    Farmer,
    Processor,
    Distributor,
    Regulator,
    Consumer,
    Auditor,
}

/// Sensor reading from IoT devices
#[contracttype]
#[derive(Clone, Debug)]
pub struct SensorReading {
    /// Sensor device ID
    pub device_id: Bytes,
    /// Sensor type
    pub sensor_type: String,
    /// Measured value
    pub value: f32,
    /// Unit of measurement
    pub unit: String,
    /// Reading timestamp
    pub timestamp: u64,
    /// Sensor location
    pub location: Location,
    /// Data quality score
    pub quality_score: u8,
    /// Additional metadata
    pub metadata: Bytes,
}

/// Alert for anomalies or issues
#[contracttype]
#[derive(Clone, Debug)]
pub struct Alert {
    /// Alert identifier
    pub id: Bytes,
    /// Alert type
    pub alert_type: AlertType,
    /// Severity level
    pub severity: AlertSeverity,
    /// Related product ID
    pub product_id: Option<Bytes>,
    /// Related shipment ID
    pub shipment_id: Option<Bytes>,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: u64,
    /// Alert source
    pub source: Address,
    /// Alert status
    pub status: AlertStatus,
    /// Resolution timestamp
    pub resolved_at: Option<u64>,
}

/// Alert type enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlertType {
    TemperatureAnomaly,
    QualityIssue,
    DelayedShipment,
    SecurityBreach,
    ComplianceViolation,
    SystemError,
    FraudDetection,
}

/// Alert severity enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Alert status enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlertStatus {
    Active,
    Investigating,
    Resolved,
    FalsePositive,
}

/// System configuration
#[contracttype]
#[derive(Clone, Debug)]
pub struct SystemConfig {
    /// Minimum stake amount
    pub min_stake: u128,
    /// Maximum batch size
    pub max_batch_size: u32,
    /// Quality threshold
    pub quality_threshold: u8,
    /// Fee percentage (basis points)
    pub fee_percentage: u32,
    /// Contract paused status
    pub paused: bool,
    /// Emergency brake status
    pub emergency_brake: bool,
    /// Governance parameters
    pub governance: GovernanceConfig,
}

/// Governance configuration
#[contracttype]
#[derive(Clone, Debug)]
pub struct GovernanceConfig {
    /// Voting period in seconds
    pub voting_period: u64,
    /// Minimum quorum percentage
    pub min_quorum_percentage: u32,
    /// Minimum participation percentage
    pub min_participation_percentage: u32,
    /// Proposal execution delay in seconds
    pub execution_delay: u64,
}

/// Proposal for governance
#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    /// Proposal identifier
    pub id: Bytes,
    /// Proposer address
    pub proposer: Address,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposal data
    pub data: Bytes,
    /// Creation timestamp
    pub created_at: u64,
    /// Voting deadline
    pub voting_deadline: u64,
    /// Current status
    pub status: ProposalStatus,
    /// Vote counts
    pub votes_for: u32,
    pub votes_against: u32,
    pub votes_abstain: u32,
}

/// Proposal type enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalType {
    ParameterChange,
    ContractUpgrade,
    FundTransfer,
    EmergencyAction,
    Other,
}

/// Proposal status enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Failed,
    Executed,
    Expired,
}

/// Vote record
#[contracttype]
#[derive(Clone, Debug)]
pub struct Vote {
    /// Voter address
    pub voter: Address,
    /// Proposal ID
    pub proposal_id: Bytes,
    /// Vote choice
    pub choice: VoteChoice,
    /// Voting power
    pub power: u32,
    /// Vote timestamp
    pub timestamp: u64,
}

/// Vote choice enumeration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

// Implementations for type conversions and validation
impl ProductCategory {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            ProductCategory::Vegetables => String::from_str(&"vegetables"),
            ProductCategory::Fruits => String::from_str(&"fruits"),
            ProductCategory::Grains => String::from_str(&"grains"),
            ProductCategory::Dairy => String::from_str(&"dairy"),
            ProductCategory::Meat => String::from_str(&"meat"),
            ProductCategory::Seafood => String::from_str(&"seafood"),
            ProductCategory::Herbs => String::from_str(&"herbs"),
            ProductCategory::Spices => String::from_str(&"spices"),
            ProductCategory::Other => String::from_str(&"other"),
        }
    }
}

impl ProductStatus {
    /// Check if product is in active state
    pub fn is_active(&self) -> bool {
        matches!(self, 
            ProductStatus::Registered | 
            ProductStatus::Growing | 
            ProductStatus::Harvested | 
            ProductStatus::Processing | 
            ProductStatus::Packaged | 
            ProductStatus::Shipped
        )
    }
    
    /// Check if product is completed
    pub fn is_completed(&self) -> bool {
        matches!(self, ProductStatus::Delivered)
    }
}

impl QualityGrade {
    /// Convert to numeric score
    pub fn to_score(&self) -> u8 {
        match self {
            QualityGrade::A => 95,
            QualityGrade::B => 85,
            QualityGrade::C => 75,
            QualityGrade::D => 65,
            QualityGrade::F => 50,
        }
    }
    
    /// Convert from numeric score
    pub fn from_score(score: u8) -> Self {
        if score >= 90 {
            QualityGrade::A
        } else if score >= 80 {
            QualityGrade::B
        } else if score >= 70 {
            QualityGrade::C
        } else if score >= 60 {
            QualityGrade::D
        } else {
            QualityGrade::F
        }
    }
}
