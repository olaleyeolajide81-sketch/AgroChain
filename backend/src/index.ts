import express from 'express'
import cors from 'cors'
import helmet from 'helmet'
import morgan from 'morgan'
import rateLimit from 'express-rate-limit'
import dotenv from 'dotenv'

import { errorHandler } from '@/middleware/errorHandler'
import { logger } from '@/utils/logger'
import { connectDatabase } from '@/config/database'
import { connectRedis } from '@/config/redis'
import { productRoutes } from '@/routes/products'
import { userRoutes } from '@/routes/users'
import { shipmentRoutes } from '@/routes/shipments'
import { qualityRoutes } from '@/routes/quality'
import { analyticsRoutes } from '@/routes/analytics'

// Load environment variables
dotenv.config()

const app = express()
const PORT = process.env.PORT || 3000

// Rate limiting
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // Limit each IP to 100 requests per window
  message: 'Too many requests from this IP, please try again later.',
})

// Middleware
app.use(helmet())
app.use(cors({
  origin: process.env.FRONTEND_URL || 'http://localhost:3100',
  credentials: true,
}))
app.use(morgan('combined', { stream: { write: (message) => logger.info(message.trim()) } }))
app.use(express.json({ limit: '10mb' }))
app.use(express.urlencoded({ extended: true }))
app.use(limiter)

// Health check endpoint
app.get('/health', (req, res) => {
  res.status(200).json({
    status: 'OK',
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
    environment: process.env.NODE_ENV || 'development',
  })
})

// API routes
app.use('/api/v1/products', productRoutes)
app.use('/api/v1/users', userRoutes)
app.use('/api/v1/shipments', shipmentRoutes)
app.use('/api/v1/quality', qualityRoutes)
app.use('/api/v1/analytics', analyticsRoutes)

// 404 handler
app.use('*', (req, res) => {
  res.status(404).json({
    error: 'Not Found',
    message: `Route ${req.originalUrl} not found`,
  })
})

// Error handling middleware
app.use(errorHandler)

// Start server
async function startServer() {
  try {
    // Connect to database
    await connectDatabase()
    logger.info('Database connected successfully')

    // Connect to Redis
    await connectRedis()
    logger.info('Redis connected successfully')

    // Start server
    app.listen(PORT, () => {
      logger.info(`🚀 Server running on port ${PORT}`)
      logger.info(`📚 API documentation: http://localhost:${PORT}/api/v1/docs`)
      logger.info(`🏥 Health check: http://localhost:${PORT}/health`)
    })
  } catch (error) {
    logger.error('Failed to start server:', error)
    process.exit(1)
  }
}

// Handle graceful shutdown
process.on('SIGTERM', () => {
  logger.info('SIGTERM received, shutting down gracefully')
  process.exit(0)
})

process.on('SIGINT', () => {
  logger.info('SIGINT received, shutting down gracefully')
  process.exit(0)
})

// Start the server
startServer()
