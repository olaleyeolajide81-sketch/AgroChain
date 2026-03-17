//! # AgroChain Product Registry Contract
//! 
//! This contract manages the registration and lifecycle of agricultural products
//! in the AgroChain decentralized supply chain system.

#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, Address, Bytes, Symbol, String, Vec, Map, u64, u128, u32, u8, bool, Env,
    log, contracterror,
};

use agrochain_shared::{
    Product, ProductStatus, ProductCategory, Location, Certificate, CertificationType,
    ContractState, Configuration, ContractError, utils, constants,
};

/// Product Registry Contract
#[contractimpl]
#[contracttype]
pub struct ProductRegistry {
    state: ContractState,
}

/// Contract events
#[contracttype]
#[derive(Debug)]
pub enum ProductEvent {
    /// Product registered
    ProductRegistered {
        product_id: Bytes,
        farmer: Address,
        name: String,
        category: ProductCategory,
    },
    /// Product status updated
    ProductStatusUpdated {
        product_id: Bytes,
        old_status: ProductStatus,
        new_status: ProductStatus,
    },
    /// Product ownership transferred
    ProductTransferred {
        product_id: Bytes,
        from: Address,
        to: Address,
    },
    /// Certificate added to product
    CertificateAdded {
        product_id: Bytes,
        certificate_id: Bytes,
        cert_type: CertificationType,
    },
    /// Product metadata updated
    ProductMetadataUpdated {
        product_id: Bytes,
        metadata_hash: Bytes,
    },
}

impl ProductRegistry {
    /// Initialize the contract
    /// 
    /// # Arguments
    /// * `admin` - The contract administrator address
    /// * `config` - Initial configuration
    pub fn initialize(env: Env, admin: Address, config: Configuration) {
        // Validate configuration
        config.validate().unwrap();
        
        // Create initial state
        let state = ContractState::new(admin);
        
        // Store state
        env.storage().instance().set(&constants::STATE_KEY, &state);
        
        log!(&env, "ProductRegistry initialized by admin: {}", admin);
    }

    /// Register a new agricultural product
    /// 
    /// # Arguments
    /// * `farmer` - The farmer's address
    /// * `name` - Product name
    /// * `category` - Product category
    /// * `origin` - Origin location
    /// * `metadata_hash` - Hash of product metadata
    /// 
    /// # Returns
    /// * `Bytes` - Unique product identifier
    pub fn register_product(
        env: Env,
        farmer: Address,
        name: String,
        category: ProductCategory,
        origin: Location,
        metadata_hash: Bytes,
    ) -> Bytes {
        // Check if contract is paused
        if utils::is_paused(&env) {
            panic!("Contract is paused");
        }
        
        // Validate inputs
        if name.is_empty() {
            panic!("Product name cannot be empty");
        }
        
        if !utils::validate_address(&farmer) {
            panic!("Invalid farmer address");
        }
        
        // Generate unique product ID
        let product_id = utils::generate_unique_id(&env, &Symbol::new(&env, "PRODUCT"));
        
        // Create product
        let product = Product {
            id: product_id.clone(),
            name: name.clone(),
            category,
            status: ProductStatus::Registered,
            farmer,
            origin: origin.clone(),
            owner: farmer,
            created_at: env.ledger().timestamp(),
            updated_at: env.ledger().timestamp(),
            metadata_hash: metadata_hash.clone(),
            quality_grade: None,
            certifications: Vec::new(&env),
            batch_id: None,
            expires_at: None,
        };
        
        // Store product
        let product_key = Symbol::new(&env, "PRODUCT");
        let mut products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap_or(Map::new(&env));
        products.set(product_id.clone(), product);
        env.storage().persistent().set(&product_key, products);
        
        // Update statistics
        let mut stats: Map<Symbol, u64> = env.storage().persistent().get(&Symbol::new(&env, "STATS")).unwrap_or(Map::new(&env));
        let total_products = stats.get(Symbol::new(&env, "TOTAL_PRODUCTS")).unwrap_or(0);
        stats.set(Symbol::new(&env, "TOTAL_PRODUCTS"), total_products + 1);
        env.storage().persistent().set(&Symbol::new(&env, "STATS"), stats);
        
        // Emit event
        env.events().publish(
            ProductEvent::ProductRegistered {
                product_id: product_id.clone(),
                farmer,
                name,
                category,
            },
        );
        
        log!(&env, "Product registered: {}", product_id);
        
        product_id
    }

    /// Update product status
    /// 
    /// # Arguments
    /// * `product_id` - Product identifier
    /// * `new_status` - New status
    /// * `updated_by` - Address updating the status
    pub fn update_status(
        env: Env,
        product_id: Bytes,
        new_status: ProductStatus,
        updated_by: Address,
    ) {
        // Check if contract is paused
        if utils::is_paused(&env) {
            panic!("Contract is paused");
        }
        
        // Get product
        let product = Self::get_product(&env, product_id.clone());
        
        // Validate permissions (only owner or admin can update status)
        let state: ContractState = env.storage().instance().get(&constants::STATE_KEY).unwrap();
        if product.owner != updated_by && state.admin != updated_by {
            panic!("Only owner or admin can update status");
        }
        
        // Validate status transition
        Self::validate_status_transition(&product.status, &new_status);
        
        // Update product
        let mut updated_product = product;
        let old_status = product.status;
        updated_product.status = new_status;
        updated_product.updated_at = env.ledger().timestamp();
        
        // Store updated product
        let product_key = Symbol::new(&env, "PRODUCT");
        let mut products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        products.set(product_id.clone(), updated_product);
        env.storage().persistent().set(&product_key, products);
        
        // Emit event
        env.events().publish(
            ProductEvent::ProductStatusUpdated {
                product_id,
                old_status,
                new_status,
            },
        );
        
        log!(&env, "Product status updated: {} -> {}", old_status, new_status);
    }

    /// Transfer product ownership
    /// 
    /// # Arguments
    /// * `product_id` - Product identifier
    /// * `to` - New owner address
    /// * `from` - Current owner address
    pub fn transfer_ownership(env: Env, product_id: Bytes, to: Address, from: Address) {
        // Check if contract is paused
        if utils::is_paused(&env) {
            panic!("Contract is paused");
        }
        
        // Get product
        let mut product = Self::get_product(&env, product_id.clone());
        
        // Validate ownership
        if product.owner != from {
            panic!("Only current owner can transfer ownership");
        }
        
        if !utils::validate_address(&to) {
            panic!("Invalid recipient address");
        }
        
        // Update ownership
        let old_owner = product.owner;
        product.owner = to;
        product.updated_at = env.ledger().timestamp();
        
        // Store updated product
        let product_key = Symbol::new(&env, "PRODUCT");
        let mut products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        products.set(product_id.clone(), product);
        env.storage().persistent().set(&product_key, products);
        
        // Emit event
        env.events().publish(
            ProductEvent::ProductTransferred {
                product_id,
                from: old_owner,
                to,
            },
        );
        
        log!(&env, "Product ownership transferred: {} -> {}", old_owner, to);
    }

    /// Update product metadata
    /// 
    /// # Arguments
    /// * `product_id` - Product identifier
    /// * `metadata_hash` - New metadata hash
    /// * `updated_by` - Address updating metadata
    pub fn update_metadata(
        env: Env,
        product_id: Bytes,
        metadata_hash: Bytes,
        updated_by: Address,
    ) {
        // Check if contract is paused
        if utils::is_paused(&env) {
            panic!("Contract is paused");
        }
        
        // Get product
        let mut product = Self::get_product(&env, product_id.clone());
        
        // Validate permissions
        if product.owner != updated_by {
            panic!("Only owner can update metadata");
        }
        
        // Update metadata
        product.metadata_hash = metadata_hash.clone();
        product.updated_at = env.ledger().timestamp();
        
        // Store updated product
        let product_key = Symbol::new(&env, "PRODUCT");
        let mut products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        products.set(product_id.clone(), product);
        env.storage().persistent().set(&product_key, products);
        
        // Emit event
        env.events().publish(
            ProductEvent::ProductMetadataUpdated {
                product_id,
                metadata_hash,
            },
        );
        
        log!(&env, "Product metadata updated: {}", product_id);
    }

    /// Add certificate to product
    /// 
    /// # Arguments
    /// * `product_id` - Product identifier
    /// * `certificate_id` - Certificate identifier
    /// * `cert_type` - Certificate type
    /// * `issuer` - Certificate issuer address
    /// * `data_hash` - Certificate data hash
    pub fn add_certificate(
        env: Env,
        product_id: Bytes,
        certificate_id: Bytes,
        cert_type: CertificationType,
        issuer: Address,
        data_hash: Bytes,
    ) {
        // Check if contract is paused
        if utils::is_paused(&env) {
            panic!("Contract is paused");
        }
        
        // Get product
        let mut product = Self::get_product(&env, product_id.clone());
        
        // Validate permissions (only owner can add certificates)
        let state: ContractState = env.storage().instance().get(&constants::STATE_KEY).unwrap();
        if product.owner != issuer && state.admin != issuer {
            panic!("Only owner or admin can add certificates");
        }
        
        // Create certificate
        let certificate = Certificate {
            id: certificate_id.clone(),
            cert_type,
            issuer,
            issued_at: env.ledger().timestamp(),
            expires_at: None, // Could be added as parameter
            data_hash: data_hash.clone(),
            verified: false, // Requires verification process
        };
        
        // Add certificate to product
        product.certifications.push_back(certificate);
        product.updated_at = env.ledger().timestamp();
        
        // Store updated product
        let product_key = Symbol::new(&env, "PRODUCT");
        let mut products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        products.set(product_id.clone(), product);
        env.storage().persistent().set(&product_key, products);
        
        // Emit event
        env.events().publish(
            ProductEvent::CertificateAdded {
                product_id,
                certificate_id,
                cert_type,
            },
        );
        
        log!(&env, "Certificate added to product: {}", certificate_id);
    }

    /// Get product information
    /// 
    /// # Arguments
    /// * `product_id` - Product identifier
    /// 
    /// # Returns
    /// * `Product` - Product information
    pub fn get_product(env: Env, product_id: Bytes) -> Product {
        let product_key = Symbol::new(&env, "PRODUCT");
        let products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        
        products.get(product_id.clone()).unwrap_or_else(|| {
            panic!("Product not found: {}", product_id);
        })
    }

    /// Get products by owner
    /// 
    /// # Arguments
    /// * `owner` - Owner address
    /// * `limit` - Maximum number of products to return
    /// * `offset` - Starting offset
    /// 
    /// # Returns
    /// * `Vec<Product>` - List of products
    pub fn get_products_by_owner(
        env: Env,
        owner: Address,
        limit: u32,
        offset: u32,
    ) -> Vec<Product> {
        let product_key = Symbol::new(&env, "PRODUCT");
        let products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        
        let mut result = Vec::new(&env);
        let mut count = 0;
        let mut skipped = 0;
        
        for (product_id, product) in products {
            if product.owner == owner {
                if skipped >= offset {
                    if count < limit {
                        result.push_back(product);
                        count += 1;
                    }
                } else {
                    skipped += 1;
                }
            }
        }
        
        result
    }

    /// Get products by farmer
    /// 
    /// # Arguments
    /// * `farmer` - Farmer address
    /// * `limit` - Maximum number of products to return
    /// * `offset` - Starting offset
    /// 
    /// # Returns
    /// * `Vec<Product>` - List of products
    pub fn get_products_by_farmer(
        env: Env,
        farmer: Address,
        limit: u32,
        offset: u32,
    ) -> Vec<Product> {
        Self::get_products_by_owner(env, farmer, limit, offset)
    }

    /// Get products by category
    /// 
    /// # Arguments
    /// * `category` - Product category
    /// * `limit` - Maximum number of products to return
    /// * `offset` - Starting offset
    /// 
    /// # Returns
    /// * `Vec<Product>` - List of products
    pub fn get_products_by_category(
        env: Env,
        category: ProductCategory,
        limit: u32,
        offset: u32,
    ) -> Vec<Product> {
        let product_key = Symbol::new(&env, "PRODUCT");
        let products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        
        let mut result = Vec::new(&env);
        let mut count = 0;
        let mut skipped = 0;
        
        for (product_id, product) in products {
            if product.category == category {
                if skipped >= offset {
                    if count < limit {
                        result.push_back(product);
                        count += 1;
                    }
                } else {
                    skipped += 1;
                }
            }
        }
        
        result
    }

    /// Get products by status
    /// 
    /// # Arguments
    /// * `status` - Product status
    /// * `limit` - Maximum number of products to return
    /// * `offset` - Starting offset
    /// 
    /// # Returns
    /// * `Vec<Product>` - List of products
    pub fn get_products_by_status(
        env: Env,
        status: ProductStatus,
        limit: u32,
        offset: u32,
    ) -> Vec<Product> {
        let product_key = Symbol::new(&env, "PRODUCT");
        let products: Map<Bytes, Product> = env.storage().persistent().get(&product_key).unwrap();
        
        let mut result = Vec::new(&env);
        let mut count = 0;
        let mut skipped = 0;
        
        for (product_id, product) in products {
            if product.status == status {
                if skipped >= offset {
                    if count < limit {
                        result.push_back(product);
                        count += 1;
                    }
                } else {
                    skipped += 1;
                }
            }
        }
        
        result
    }

    /// Get total number of products
    /// 
    /// # Returns
    /// * `u64` - Total product count
    pub fn get_total_products(env: Env) -> u64 {
        let stats: Map<Symbol, u64> = env.storage().persistent().get(&Symbol::new(&env, "STATS")).unwrap();
        stats.get(Symbol::new(&env, "TOTAL_PRODUCTS")).unwrap_or(0)
    }

    /// Get contract configuration
    /// 
    /// # Returns
    /// * `Configuration` - Current configuration
    pub fn get_config(env: Env) -> Configuration {
        let state: ContractState = env.storage().instance().get(&constants::STATE_KEY).unwrap();
        state.config
    }

    /// Update contract configuration (admin only)
    /// 
    /// # Arguments
    /// * `new_config` - New configuration
    /// * `admin` - Admin address
    pub fn update_config(env: Env, new_config: Configuration, admin: Address) {
        let state: ContractState = env.storage().instance().get(&constants::STATE_KEY).unwrap();
        
        if state.admin != admin {
            panic!("Only admin can update configuration");
        }
        
        // Validate new configuration
        new_config.validate().unwrap();
        
        // Update state
        let mut updated_state = state;
        updated_state.config = new_config;
        env.storage().instance().set(&constants::STATE_KEY, updated_state);
        
        log!(&env, "Configuration updated by admin: {}", admin);
    }

    /// Validate status transition
    fn validate_status_transition(old_status: &ProductStatus, new_status: &ProductStatus) {
        // Define valid transitions
        match old_status {
            ProductStatus::Registered => {
                if !matches!(new_status, ProductStatus::Growing | ProductStatus::Harvested) {
                    panic!("Invalid status transition from Registered");
                }
            }
            ProductStatus::Growing => {
                if !matches!(new_status, ProductStatus::Harvested | ProductStatus::Processing) {
                    panic!("Invalid status transition from Growing");
                }
            }
            ProductStatus::Harvested => {
                if !matches!(new_status, ProductStatus::Processing | ProductStatus::Packaged) {
                    panic!("Invalid status transition from Harvested");
                }
            }
            ProductStatus::Processing => {
                if !matches!(new_status, ProductStatus::Packaged | ProductStatus::Shipped) {
                    panic!("Invalid status transition from Processing");
                }
            }
            ProductStatus::Packaged => {
                if !matches!(new_status, ProductStatus::Shipped) {
                    panic!("Invalid status transition from Packaged");
                }
            }
            ProductStatus::Shipped => {
                if !matches!(new_status, ProductStatus::Delivered | ProductStatus::Delayed | ProductStatus::Lost) {
                    panic!("Invalid status transition from Shipped");
                }
            }
            ProductStatus::Delivered => {
                if !matches!(new_status, ProductStatus::Delivered | ProductStatus::Recalled) {
                    panic!("Invalid status transition from Delivered");
                }
            }
            ProductStatus::Recalled => {
                if !matches!(new_status, ProductStatus::Recalled | ProductStatus::Expired) {
                    panic!("Invalid status transition from Recalled");
                }
            }
            ProductStatus::Expired => {
                if !matches!(new_status, ProductStatus::Expired) {
                    panic!("Invalid status transition from Expired");
                }
            }
        }
    }
}
