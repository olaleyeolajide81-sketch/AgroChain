import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { SorobanReactProvider } from '@soroban-react/core'
import { FreighterConnector } from '@soroban-react/freighter'
import { Toaster } from 'sonner'

import App from './App'
import './index.css'

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 3,
      staleTime: 5 * 60 * 1000, // 5 minutes
    },
  },
})

// Soroban React connectors
const connectors = [new FreighterConnector()]

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <SorobanReactProvider connectors={connectors}>
        <BrowserRouter>
          <App />
          <Toaster />
        </BrowserRouter>
      </SorobanReactProvider>
    </QueryClientProvider>
  </React.StrictMode>,
)
