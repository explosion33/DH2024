import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import './index.css'


const config = getConfig();

const providerConfig = {
  domain: config.domain,
  clientId: config.clientId,
  onRedirectCallback,
  authorizationParams: {
    redirect_uri: window.location.origin,
    ...(config.audience ? { audience: config.audience } : null),
  },
};

createRoot(document.getElementById('root')).render(
    <StrictMode>
      <Auth0Provider
        {...providerConfig}
      >
        <App />

      </Auth0Provider>
    </StrictMode>,
  )
