#!/bin/bash

# AgroChain Development Environment Setup Script
# This script sets up complete development environment for AgroChain

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on supported platform
check_platform() {
    log_info "Checking platform compatibility..."
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        PLATFORM="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        PLATFORM="macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        PLATFORM="windows"
    else
        log_error "Unsupported platform: $OSTYPE"
        exit 1
    fi
    
    log_success "Platform detected: $PLATFORM"
}

# Check system requirements
check_requirements() {
    log_info "Checking system requirements..."
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js is not installed. Please install Node.js 18+"
        exit 1
    fi
    
    NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
    if [ "$NODE_VERSION" -lt 18 ]; then
        log_error "Node.js version 18+ is required. Current version: $(node -v)"
        exit 1
    fi
    log_success "Node.js $(node -v) detected"
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        log_error "npm is not installed"
        exit 1
    fi
    log_success "npm $(npm -v) detected"
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        log_warning "Rust is not installed. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    else
        log_success "Rust $(rustc --version) detected"
    fi
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_warning "Docker is not installed. Please install Docker for full functionality"
    else
        log_success "Docker $(docker --version) detected"
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
        log_warning "Docker Compose is not installed. Please install Docker Compose"
    else
        log_success "Docker Compose detected"
    fi
}

# Install dependencies
install_dependencies() {
    log_info "Installing project dependencies..."
    
    # Install Node.js dependencies
    log_info "Installing Node.js dependencies..."
    npm ci
    
    # Install Rust dependencies
    log_info "Building Rust contracts..."
    cd contracts && cargo build && cd ..
    
    # Install Python dependencies for IoT
    if command -v python3 &> /dev/null; then
        log_info "Installing Python dependencies for IoT..."
        cd iot && pip3 install -r requirements.txt && cd ..
    else
        log_warning "Python3 not found. IoT components may not work"
    fi
    
    log_success "Dependencies installed successfully"
}

# Setup environment configuration
setup_environment() {
    log_info "Setting up environment configuration..."
    
    if [ ! -f .env ]; then
        cp .env.example .env
        log_success "Environment configuration created (.env)"
        log_warning "Please edit .env file with your configuration"
    else
        log_info "Environment configuration already exists"
    fi
}

# Setup database
setup_database() {
    log_info "Setting up database..."
    
    if command -v docker &> /dev/null; then
        # Start PostgreSQL and Redis
        log_info "Starting database services..."
        docker-compose up -d postgres redis
        
        # Wait for services to be ready
        log_info "Waiting for database services to be ready..."
        sleep 10
        
        # Run database migrations
        log_info "Running database migrations..."
        cd backend && npm run migrate || log_warning "Database migrations failed or not available" && cd ..
        
        log_success "Database setup completed"
    else
        log_warning "Docker not available. Please set up database manually"
    fi
}

# Install development tools
install_dev_tools() {
    log_info "Installing development tools..."
    
    # Install Stellar CLI
    if ! command -v stellar &> /dev/null; then
        log_info "Installing Stellar CLI..."
        cargo install stellar-cli --locked
    fi
    
    # Install Soroban CLI
    if ! command -v soroban &> /dev/null; then
        log_info "Installing Soroban CLI..."
        cargo install soroban-cli --locked
    fi
    
    # Install global Node.js tools
    log_info "Installing global Node.js tools..."
    npm install -g @commitlint/cli @commitlint/config-conventional
    
    log_success "Development tools installed"
}

# Setup Git hooks
setup_git_hooks() {
    log_info "Setting up Git hooks..."
    
    # Install husky
    npm install --save-dev husky
    
    # Initialize husky
    npx husky install
    
    # Add pre-commit hook
    npx husky add .husky/pre-commit "npm run lint && npm run test:unit"
    
    # Add commit-msg hook
    npx husky add .husky/commit-msg "npx commitlint --edit $1"
    
    log_success "Git hooks setup completed"
}

# Create development certificates
setup_certificates() {
    log_info "Setting up development certificates..."
    
    # Create certificates directory
    mkdir -p certificates
    
    # Generate self-signed certificate for development
    if command -v openssl &> /dev/null; then
        openssl req -x509 -newkey rsa:4096 -keyout certificates/key.pem -out certificates/cert.pem -days 365 -nodes -subj "/C=US/ST=CA/L=San Francisco/O=AgroChain/OU=Development/CN=localhost"
        log_success "Development certificates created"
    else
        log_warning "OpenSSL not found. Skipping certificate generation"
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    # Check if all services can start
    log_info "Testing service startup..."
    
    # Test contracts build
    cd contracts && cargo check && cd ..
    log_success "Smart contracts build successfully"
    
    # Test backend build
    cd backend && npm run build && cd ..
    log_success "Backend builds successfully"
    
    # Test frontend build
    cd frontend && npm run build && cd ..
    log_success "Frontend builds successfully"
    
    log_success "Installation verification completed"
}

# Print next steps
print_next_steps() {
    log_info "Setup completed successfully!"
    echo ""
    echo -e "${GREEN}🎉 AgroChain development environment is ready!${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "1. Edit .env file with your configuration"
    echo "2. Start development environment: npm run dev"
    echo "3. Open your browser and navigate to applications"
    echo "4. Read the contributing guidelines: CONTRIBUTING.md"
    echo ""
    echo -e "${BLUE}Useful commands:${NC}"
    echo "  npm run dev              - Start all development services"
    echo "  npm run test             - Run all tests"
    echo "  npm run lint             - Run code quality checks"
    echo "  npm run docker:up         - Start Docker services"
    echo "  npm run docker:down       - Stop Docker services"
    echo ""
    echo -e "${BLUE}Application URLs:${NC}"
    echo "  Farmer Dashboard: http://localhost:3100"
    echo "  API Gateway: http://localhost:3000"
    echo "  API Documentation: http://localhost:3000/api-docs"
    echo ""
    echo -e "${YELLOW}Note: Make sure Docker is running for full functionality${NC}"
}

# Main setup function
main() {
    echo -e "${GREEN}🌱 AgroChain Development Environment Setup${NC}"
    echo "=========================================="
    echo ""
    
    check_platform
    check_requirements
    install_dependencies
    setup_environment
    setup_database
    install_dev_tools
    setup_git_hooks
    setup_certificates
    verify_installation
    print_next_steps
    
    echo ""
    echo -e "${GREEN}✅ Setup completed successfully!${NC}"
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "AgroChain Development Environment Setup"
        echo ""
        echo "Usage: $0 [options]"
        echo ""
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --skip-docker  Skip Docker setup"
        echo "  --dev-only     Only install development dependencies"
        echo ""
        exit 0
        ;;
    --skip-docker)
        log_warning "Skipping Docker setup"
        setup_database() { log_info "Skipping database setup"; }
        main
        ;;
    --dev-only)
        log_info "Running development-only setup"
        install_dependencies
        setup_environment
        install_dev_tools
        setup_git_hooks
        log_success "Development setup completed"
        ;;
    *)
        main
        ;;
esac
