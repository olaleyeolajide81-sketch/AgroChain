import dotenv from 'dotenv'

// Load test environment variables
dotenv.config({ path: '../.env.test' })

// Set test environment
process.env.NODE_ENV = 'test'

// Mock console methods for cleaner test output
global.console = {
  ...console,
  log: jest.fn(),
  debug: jest.fn(),
  info: jest.fn(),
  warn: jest.fn(),
  error: jest.fn(),
}

// Setup global test timeout
jest.setTimeout(30000)

// Mock external services
jest.mock('@soroban-react/core', () => ({
  SorobanReactProvider: ({ children }: { children: React.ReactNode }) => children,
  useSorobanReact: () => ({
    address: 'test-address',
    connect: jest.fn(),
    disconnect: jest.fn(),
  }),
}))

// Mock Stellar SDK
jest.mock('soroban-client', () => ({
  Server: jest.fn(),
  TransactionBuilder: jest.fn(),
  Networks: {
    TESTNET: 'testnet',
    FUTURENET: 'futurenet',
  },
}))

// Global test utilities
global.testUtils = {
  generateTestAddress: () => `test-${Math.random().toString(36).substr(2, 9)}`,
  generateTestProductId: () => `product-${Math.random().toString(36).substr(2, 9)}`,
  generateTestShipmentId: () => `shipment-${Math.random().toString(36).substr(2, 9)}`,
}
