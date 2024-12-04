import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { clusterApiUrl } from '@solana/web3.js';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import '@solana/wallet-adapter-react-ui/styles.css';

// Theme
import theme from './theme';

// Components
import Header from './components/layout/Header';
import Landing from './pages/Landing';
import Dashboard from './pages/Dashboard';
import CreateToken from './pages/CreateToken';
import TokenSwap from './pages/TokenSwap';
import TokenStaking from './pages/TokenStaking';
import DeFi from './pages/DeFi';

// Initialize wallet adapter
const wallets = [new PhantomWalletAdapter()];
const endpoint = clusterApiUrl('devnet');

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <ConnectionProvider endpoint={endpoint}>
        <WalletProvider wallets={wallets} autoConnect>
          <WalletModalProvider>
            <Router>
              <div style={{ 
                minHeight: '100vh',
                background: theme.palette.background.default 
              }}>
                <Header />
                <Routes>
                  <Route path="/" element={<Landing />} />
                  <Route path="/dashboard" element={<Dashboard />} />
                  <Route path="/create" element={<CreateToken />} />
                  <Route path="/swap" element={<TokenSwap />} />
                  <Route path="/staking" element={<TokenStaking />} />
                  <Route path="/defi" element={<DeFi />} />
                </Routes>
              </div>
            </Router>
            <ToastContainer
              position="bottom-right"
              autoClose={5000}
              hideProgressBar={false}
              newestOnTop
              closeOnClick
              rtl={false}
              pauseOnFocusLoss
              draggable
              pauseOnHover
              theme="dark"
            />
          </WalletModalProvider>
        </WalletProvider>
      </ConnectionProvider>
    </ThemeProvider>
  );
}

export default App;
