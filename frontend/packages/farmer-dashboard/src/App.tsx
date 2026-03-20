import { Routes, Route } from 'react-router-dom'
import { Layout } from './components/Layout'
import { Dashboard } from './pages/Dashboard'
import { ProductRegistration } from './pages/ProductRegistration'
import { ProductList } from './pages/ProductList'
import { ProductDetail } from './pages/ProductDetail'
import { Settings } from './pages/Settings'
import { Profile } from './pages/Profile'

function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/products" element={<ProductList />} />
        <Route path="/products/register" element={<ProductRegistration />} />
        <Route path="/products/:id" element={<ProductDetail />} />
        <Route path="/settings" element={<Settings />} />
        <Route path="/profile" element={<Profile />} />
      </Routes>
    </Layout>
  )
}

export default App
