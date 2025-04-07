import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { LoginPage } from './pages/login/LoginPage'
import { UiPage } from './utils/UiPage'
import { ToastContainer } from 'react-toastify'

function App() {
  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route path="/login" element={<UiPage><LoginPage /></UiPage>} />
        </Routes>
      </BrowserRouter>
      <ToastContainer />
    </>
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
