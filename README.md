# 🌱 AgroChain: Decentralized Agricultural Supply Chain Tracking

[![Build Status](https://github.com/olaleyeolajide81-sketch/AgroChain/workflows/CI/badge.svg)
[![Coverage](https://img.shields.io/badge/coverage-85%25-green)
[![License](https://img.shields.io/badge/license-MIT-blue)
[![Stellar](https://img.shields.io/badge/blockchain-Stellar-purple)]

A comprehensive decentralized agricultural supply chain tracking system built on the Stellar blockchain, providing transparency, traceability, and quality assurance from farm to table.

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/olaleyeolajide81-sketch/AgroChain.git
cd AgroChain

# Run setup script (Linux/macOS)
chmod +x scripts/setup.sh && ./scripts/setup.sh

# Or setup manually (Windows)
npm run setup:dev

# Start development environment
npm run dev
```

## 📋 Features

### 🌾 Blockchain Integration
- **Stellar Smart Contracts** - Immutable supply chain records
- **Soroban Platform** - Modern smart contract development
- **Cross-border Payments** - Global agricultural transactions
- **Tokenized Assets** - Digital representation of agricultural products

### 📊 Supply Chain Management
- **Product Registration** - Farm-to-fork tracking
- **Quality Assurance** - Real-time monitoring and certification
- **Batch Tracking** - Lot-based traceability
- **Temperature Monitoring** - Cold chain integrity

### 🎯 User Applications
- **Farmer Dashboard** - Product registration and management
- **Processor Portal** - Food processing workflow
- **Distributor Platform** - Logistics and distribution
- **Consumer App** - Product origin and quality verification

### 🔌 IoT Integration
- **Sensor Networks** - Real-time environmental monitoring
- **GPS Tracking** - Location-based traceability
- **Temperature Sensors** - Cold chain monitoring
- **Quality Sensors** - Automated quality assessment

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Farmers      │    │   Processors   │    │  Distributors   │
│                │    │                │    │                │
│ 🌱📱💻       │◄──►│ 🏭📱💻       │◄──►│ 🚚📱💻       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                      │                      │
         └──────────────────────┼──────────────────────┘
                                │
                    ┌─────────────────┐
                    │  Stellar        │
                    │  Blockchain     │
                    │  ⭐            │
                    └─────────────────┘
                                │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│   Regulators   │ │   Consumers    │ │   IoT Devices   │
│                │ │                │ │                │
│ 🏛️📱💻       │ │ 🛒📱💻       │ │ 🌡️📡🔋       │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

## 🛠️ Technology Stack

### Blockchain Layer
- **Stellar** - Fast, low-cost blockchain platform
- **Soroban** - Smart contract platform for Stellar
- **Rust** - Secure smart contract development

### Backend Services
- **Node.js** - JavaScript runtime for backend
- **TypeScript** - Type-safe development
- **Express.js** - Web framework for APIs
- **PostgreSQL** - Primary database
- **Redis** - Caching and session storage

### Frontend Applications
- **React** - Modern UI framework
- **TypeScript** - Type-safe frontend development
- **Vite** - Fast build tool
- **Tailwind CSS** - Utility-first styling
- **React Query** - Server state management

### IoT Infrastructure
- **Python** - IoT gateway development
- **MQTT** - Device communication protocol
- **Docker** - Container orchestration
- **Kubernetes** - Container orchestration

## 📁 Project Structure

```
agrochain/
├── 📋 README.md                 # Project overview
├── 📄 LICENSE                   # MIT license
├── 📝 CONTRIBUTING.md           # Contribution guidelines
├── 📜 CODE_OF_CONDUCT.md         # Community standards
├── 📦 package.json             # Workspace configuration
├── 🦀 Cargo.toml               # Rust workspace
├── 🐳 docker-compose.yml        # Development environment
├── ⚙️ .env.example             # Environment template
├── 🔧 scripts/                 # Setup and deployment scripts
├── 📚 docs/                    # Documentation
├── 🧪 tests/                   # Testing framework
├── 📁 contracts/               # Stellar smart contracts
├── 🔌 backend/                 # Node.js microservices
├── 🎨 frontend/                # React applications
├── 🌐 iot/                     # IoT gateway software
├── ☁️ infrastructure/          # Kubernetes configs
└── 🤖 .github/                 # CI/CD and templates
```

## 🚀 Development

### Prerequisites
- **Node.js** 18+
- **Rust** 1.70+
- **Docker** & Docker Compose
- **Git**

### Setup Commands
```bash
# Install all dependencies
npm run setup:dev

# Start development environment
npm run dev

# Run tests
npm run test

# Build for production
npm run build

# Deploy to staging
npm run deploy:staging
```

## 🧪 Testing

```bash
# Run all tests
npm run test

# Run specific test suites
npm run test:contracts    # Smart contracts
npm run test:backend      # Backend services
npm run test:frontend     # Frontend applications
npm run test:iot          # IoT components

# Run with coverage
npm run test:coverage

# Run E2E tests
npm run test:e2e
```

## 🚀 Deployment

### Docker
```bash
# Build and run all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Kubernetes
```bash
# Deploy to staging
./scripts/deploy.sh --env staging

# Deploy to production
./scripts/deploy.sh --env production --version v1.0.0
```

## 📊 Applications

### Farmer Dashboard
- **URL**: http://localhost:3100
- **Features**: Product registration, batch management, quality tracking
- **Tech**: React, TypeScript, Tailwind CSS

### Processor Portal
- **URL**: http://localhost:3200
- **Features**: Order processing, quality control, certification
- **Tech**: React, TypeScript, Material UI

### API Gateway
- **URL**: http://localhost:3000
- **Features**: RESTful APIs, authentication, rate limiting
- **Tech**: Node.js, Express, TypeScript

### IoT Gateway
- **URL**: http://localhost:4000
- **Features**: Sensor data ingestion, anomaly detection
- **Tech**: Python, MQTT, AsyncIO

## 📖 Documentation

- **Getting Started**: [docs/getting-started/](docs/getting-started/)
- **API Reference**: [docs/api/](docs/api/)
- **Architecture**: [docs/architecture/](docs/architecture/)
- **Deployment**: [docs/deployment/](docs/deployment/)

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guide](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

### How to Contribute
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Stellar Development Foundation** - Blockchain infrastructure
- **Soroban Team** - Smart contract platform
- **Open Source Community** - Tools and libraries

## 📞 Contact

- **Issues**: [GitHub Issues](https://github.com/olaleyeolajide81-sketch/AgroChain/issues)
- **Discussions**: [GitHub Discussions](https://github.com/olaleyeolajide81-sketch/AgroChain/discussions)
- **Email**: info@agrochain.io

---

**🌱 Building transparent, sustainable agricultural supply chains together!**
