import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import './index.css'
import { Auth0Provider } from '@auth0/auth0-react';



createRoot(document.getElementById('root')).render(
  <StrictMode>
    <Auth0Provider
      domain="dev-xzc0y1vwxvcjnrg1.us.auth0.com"
      clientId="PmMWhDN8jvKXXoqLOxwCTEL1lPwlnYAc"
      authorizationParams={{
        redirect_uri: window.location.origin
      }}
    >
      <App />

    </Auth0Provider>
  </StrictMode>,
)
