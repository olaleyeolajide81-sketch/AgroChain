#!/bin/bash

# AgroChain Deployment Script
# This script handles deployment of AgroChain to different environments

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
ENVIRONMENT="staging"
VERSION="latest"
SKIP_TESTS=false
SKIP_BUILD=false
PUSH_IMAGES=true

# Configuration
REGISTRY="ghcr.io/agrochain"
PROJECT_NAME="agrochain"

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

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --env|-e)
                ENVIRONMENT="$2"
                shift 2
                ;;
            --version|-v)
                VERSION="$2"
                shift 2
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --skip-build)
                SKIP_BUILD=true
                shift
                ;;
            --no-push)
                PUSH_IMAGES=false
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show help
show_help() {
    echo "AgroChain Deployment Script"
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --env, -e <env>        Target environment (staging|production) [default: staging]"
    echo "  --version, -v <version> Version to deploy [default: latest]"
    echo "  --skip-tests          Skip running tests before deployment"
    echo "  --skip-build          Skip building Docker images"
    echo "  --no-push             Skip pushing Docker images to registry"
    echo "  --help, -h            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --env staging"
    echo "  $0 --env production --version v1.2.3"
    echo "  $0 --env staging --skip-tests --no-push"
}

# Validate environment
validate_environment() {
    case $ENVIRONMENT in
        staging|production)
            ;;
        *)
            log_error "Invalid environment: $ENVIRONMENT. Must be 'staging' or 'production'"
            exit 1
            ;;
    esac
    
    log_info "Deploying to environment: $ENVIRONMENT"
    log_info "Version: $VERSION"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    # Check kubectl
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl is not installed"
        exit 1
    fi
    
    # Check if logged into container registry
    if [ "$PUSH_IMAGES" = true ]; then
        if ! docker info | grep -q "Username"; then
            log_error "Not logged into container registry. Run 'docker login' first"
            exit 1
        fi
    fi
    
    # Check Kubernetes connection
    if ! kubectl cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Run tests
run_tests() {
    if [ "$SKIP_TESTS" = true ]; then
        log_warning "Skipping tests as requested"
        return
    fi
    
    log_info "Running tests..."
    
    # Run unit tests
    log_info "Running unit tests..."
    npm run test || {
        log_error "Unit tests failed"
        exit 1
    }
    
    # Run integration tests
    log_info "Running integration tests..."
    npm run test:integration || {
        log_error "Integration tests failed"
        exit 1
    }
    
    # Run contract tests
    log_info "Running smart contract tests..."
    cd contracts && cargo test || {
        log_error "Smart contract tests failed"
        exit 1
    }
    cd ..
    
    log_success "All tests passed"
}

# Build Docker images
build_images() {
    if [ "$SKIP_BUILD" = true ]; then
        log_warning "Skipping Docker build as requested"
        return
    fi
    
    log_info "Building Docker images..."
    
    # Build API Gateway
    log_info "Building API Gateway..."
    docker build -t $REGISTRY/api-gateway:$VERSION ./backend/api-gateway || {
        log_error "Failed to build API Gateway"
        exit 1
    }
    
    # Build Auth Service
    log_info "Building Auth Service..."
    docker build -t $REGISTRY/auth-service:$VERSION ./backend/auth-service || {
        log_error "Failed to build Auth Service"
        exit 1
    }
    
    # Build Oracle Service
    log_info "Building Oracle Service..."
    docker build -t $REGISTRY/oracle-service:$VERSION ./backend/oracle-service || {
        log_error "Failed to build Oracle Service"
        exit 1
    }
    
    # Build Notification Service
    log_info "Building Notification Service..."
    docker build -t $REGISTRY/notification-service:$VERSION ./backend/notification-service || {
        log_error "Failed to build Notification Service"
        exit 1
    }
    
    # Build Farmer Dashboard
    log_info "Building Farmer Dashboard..."
    docker build -t $REGISTRY/farmer-dashboard:$VERSION ./frontend/farmer-dashboard || {
        log_error "Failed to build Farmer Dashboard"
        exit 1
    }
    
    # Build Processor Portal
    log_info "Building Processor Portal..."
    docker build -t $REGISTRY/processor-portal:$VERSION ./frontend/processor-portal || {
        log_error "Failed to build Processor Portal"
        exit 1
    }
    
    # Build IoT Gateway
    log_info "Building IoT Gateway..."
    docker build -t $REGISTRY/iot-gateway:$VERSION ./iot/gateway-software || {
        log_error "Failed to build IoT Gateway"
        exit 1
    }
    
    log_success "All Docker images built successfully"
}

# Push Docker images
push_images() {
    if [ "$PUSH_IMAGES" = false ]; then
        log_warning "Skipping Docker push as requested"
        return
    fi
    
    log_info "Pushing Docker images to registry..."
    
    # Push all images
    docker push $REGISTRY/api-gateway:$VERSION || {
        log_error "Failed to push API Gateway"
        exit 1
    }
    
    docker push $REGISTRY/auth-service:$VERSION || {
        log_error "Failed to push Auth Service"
        exit 1
    }
    
    docker push $REGISTRY/oracle-service:$VERSION || {
        log_error "Failed to push Oracle Service"
        exit 1
    }
    
    docker push $REGISTRY/notification-service:$VERSION || {
        log_error "Failed to push Notification Service"
        exit 1
    }
    
    docker push $REGISTRY/farmer-dashboard:$VERSION || {
        log_error "Failed to push Farmer Dashboard"
        exit 1
    }
    
    docker push $REGISTRY/processor-portal:$VERSION || {
        log_error "Failed to push Processor Portal"
        exit 1
    }
    
    docker push $REGISTRY/iot-gateway:$VERSION || {
        log_error "Failed to push IoT Gateway"
        exit 1
    }
    
    log_success "All Docker images pushed successfully"
}

# Deploy smart contracts
deploy_contracts() {
    log_info "Deploying smart contracts..."
    
    # Check if Stellar CLI is installed
    if ! command -v stellar &> /dev/null; then
        log_error "Stellar CLI not found. Please install it first"
        exit 1
    fi
    
    # Deploy contracts
    cd contracts
    
    # Set environment variables
    export STELLAR_NETWORK="${STELLAR_NETWORK:-testnet}"
    export STELLAR_SECRET_KEY="${STELLAR_SECRET_KEY}"
    
    if [ -z "$STELLAR_SECRET_KEY" ]; then
        log_error "STELLAR_SECRET_KEY environment variable is required"
        exit 1
    fi
    
    # Run deployment script
    chmod +x scripts/deploy.sh
    ./scripts/deploy.sh || {
        log_error "Smart contract deployment failed"
        exit 1
    }
    
    cd ..
    log_success "Smart contracts deployed successfully"
}

# Deploy to Kubernetes
deploy_kubernetes() {
    log_info "Deploying to Kubernetes..."
    
    # Set namespace
    kubectl config set-context --current --namespace=$PROJECT_NAME-$ENVIRONMENT
    
    # Update image tags in deployment files
    log_info "Updating image tags in deployment files..."
    
    sed -i.bak "s|image: $REGISTRY/api-gateway:.*|image: $REGISTRY/api-gateway:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/api-gateway.yaml
    sed -i.bak "s|image: $REGISTRY/auth-service:.*|image: $REGISTRY/auth-service:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/auth-service.yaml
    sed -i.bak "s|image: $REGISTRY/oracle-service:.*|image: $REGISTRY/oracle-service:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/oracle-service.yaml
    sed -i.bak "s|image: $REGISTRY/notification-service:.*|image: $REGISTRY/notification-service:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/notification-service.yaml
    sed -i.bak "s|image: $REGISTRY/farmer-dashboard:.*|image: $REGISTRY/farmer-dashboard:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/farmer-dashboard.yaml
    sed -i.bak "s|image: $REGISTRY/processor-portal:.*|image: $REGISTRY/processor-portal:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/processor-portal.yaml
    sed -i.bak "s|image: $REGISTRY/iot-gateway:.*|image: $REGISTRY/iot-gateway:$VERSION|g" infrastructure/kubernetes/$ENVIRONMENT/iot-gateway.yaml
    
    # Apply deployments
    log_info "Applying Kubernetes deployments..."
    
    # Apply namespace
    kubectl apply -f infrastructure/kubernetes/$ENVIRONMENT/namespace.yaml || {
        log_error "Failed to apply namespace"
        exit 1
    }
    
    # Apply configurations
    kubectl apply -f infrastructure/kubernetes/$ENVIRONMENT/configmaps/ || {
        log_error "Failed to apply configmaps"
        exit 1
    }
    
    kubectl apply -f infrastructure/kubernetes/$ENVIRONMENT/secrets/ || {
        log_error "Failed to apply secrets"
        exit 1
    }
    
    # Apply services
    kubectl apply -f infrastructure/kubernetes/$ENVIRONMENT/services/ || {
        log_error "Failed to apply services"
        exit 1
    }
    
    # Apply deployments
    kubectl apply -f infrastructure/kubernetes/$ENVIRONMENT/deployments/ || {
        log_error "Failed to apply deployments"
        exit 1
    }
    
    # Apply ingress
    kubectl apply -f infrastructure/kubernetes/$ENVIRONMENT/ingress/ || {
        log_error "Failed to apply ingress"
        exit 1
    }
    
    # Wait for deployments to be ready
    log_info "Waiting for deployments to be ready..."
    
    kubectl rollout status deployment/api-gateway --timeout=300s || {
        log_error "API Gateway deployment failed"
        exit 1
    }
    
    kubectl rollout status deployment/auth-service --timeout=300s || {
        log_error "Auth Service deployment failed"
        exit 1
    }
    
    kubectl rollout status deployment/farmer-dashboard --timeout=300s || {
        log_error "Farmer Dashboard deployment failed"
        exit 1
    }
    
    log_success "Kubernetes deployment completed"
}

# Run smoke tests
run_smoke_tests() {
    log_info "Running smoke tests..."
    
    # Wait for services to be ready
    sleep 30
    
    # Get service URLs
    API_URL=$(kubectl get ingress agrochain-api -o jsonpath='{.spec.rules[0].host}')
    DASHBOARD_URL=$(kubectl get ingress agrochain-dashboard -o jsonpath='{.spec.rules[0].host}')
    
    log_info "Testing API endpoint: https://$API_URL/health"
    
    # Test API health endpoint
    if curl -f -s "https://$API_URL/health" > /dev/null; then
        log_success "API health check passed"
    else
        log_error "API health check failed"
        exit 1
    fi
    
    # Test dashboard
    log_info "Testing dashboard: https://$DASHBOARD_URL"
    
    if curl -f -s "https://$DASHBOARD_URL" > /dev/null; then
        log_success "Dashboard health check passed"
    else
        log_error "Dashboard health check failed"
        exit 1
    fi
    
    log_success "Smoke tests passed"
}

# Cleanup
cleanup() {
    log_info "Cleaning up temporary files..."
    
    # Remove backup files created by sed
    find infrastructure/kubernetes/$ENVIRONMENT -name "*.bak" -delete
    
    log_success "Cleanup completed"
}

# Show deployment summary
show_summary() {
    log_info "Deployment Summary"
    echo "=================="
    echo "Environment: $ENVIRONMENT"
    echo "Version: $VERSION"
    echo "Registry: $REGISTRY"
    echo ""
    echo "Deployed Services:"
    echo "- API Gateway: https://$(kubectl get ingress agrochain-api -o jsonpath='{.spec.rules[0].host}')"
    echo "- Farmer Dashboard: https://$(kubectl get ingress agrochain-dashboard -o jsonpath='{.spec.rules[0].host}')"
    echo "- Processor Portal: https://$(kubectl get ingress agrochain-processor -o jsonpath='{.spec.rules[0].host}')"
    echo ""
    echo "Useful Commands:"
    echo "- View pods: kubectl get pods -n $PROJECT_NAME-$ENVIRONMENT"
    echo "- View logs: kubectl logs -f deployment/api-gateway -n $PROJECT_NAME-$ENVIRONMENT"
    echo "- Scale deployment: kubectl scale deployment api-gateway --replicas=3 -n $PROJECT_NAME-$ENVIRONMENT"
}

# Main deployment function
main() {
    echo -e "${GREEN}🚀 AgroChain Deployment Script${NC}"
    echo "==============================="
    echo ""
    
    parse_args "$@"
    validate_environment
    check_prerequisites
    run_tests
    build_images
    push_images
    deploy_contracts
    deploy_kubernetes
    run_smoke_tests
    cleanup
    show_summary
    
    echo ""
    echo -e "${GREEN}✅ Deployment completed successfully!${NC}"
}

# Handle script interruption
trap cleanup EXIT

# Run main function
main "$@"
