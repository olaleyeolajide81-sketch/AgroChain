module.exports = {
  displayName: 'unit',
  testEnvironment: 'node',
  roots: ['<rootDir>/unit'],
  testMatch: [
    '**/__tests__/**/*.ts',
    '**/?(*.)+(spec|test).ts'
  ],
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
  collectCoverageFrom: [
    '../**/*.ts',
    '!../**/*.d.ts',
    '!../node_modules/**',
    '!../tests/**',
    '!../dist/**',
  ],
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov', 'html'],
  setupFilesAfterEnv: ['<rootDir>/unit/setup.ts'],
  moduleNameMapping: {
    '^@/(.*)$': '<rootDir>/../$1',
  },
  testTimeout: 10000,
  verbose: true,
}
