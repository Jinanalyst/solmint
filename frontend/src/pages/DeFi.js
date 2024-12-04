import React, { useState } from 'react';
import {
    Box,
    Tabs,
    Tab,
    Typography,
    Alert,
    AlertTitle,
} from '@mui/material';
import { useWallet } from '@solana/wallet-adapter-react';
import LiquidityPool from '../components/defi/LiquidityPool';
import StakingPool from '../components/defi/Staking';
import LendingPool from '../components/defi/LendingPool';

// Mock token data - replace with actual token data from your app
const mockToken = {
    symbol: 'SOL',
    name: 'Solana',
    decimals: 9,
    address: 'So11111111111111111111111111111111111111111',
};

const DeFi = () => {
    const { publicKey } = useWallet();
    const [activeTab, setActiveTab] = useState(0);

    const handleTabChange = (event, newValue) => {
        setActiveTab(newValue);
    };

    if (!publicKey) {
        return (
            <Box sx={{ mt: 4 }}>
                <Alert severity="info">
                    <AlertTitle>Connect Wallet</AlertTitle>
                    Please connect your wallet to access DeFi features
                </Alert>
            </Box>
        );
    }

    return (
        <Box>
            <Typography variant="h4" gutterBottom fontWeight="bold">
                DeFi Hub
            </Typography>

            <Tabs value={activeTab} onChange={handleTabChange} sx={{ mb: 4 }}>
                <Tab label="Liquidity Pools" />
                <Tab label="Staking" />
                <Tab label="Lending" />
            </Tabs>

            {activeTab === 0 && (
                <LiquidityPool
                    tokenA={mockToken}
                    tokenB={{ ...mockToken, symbol: 'USDC', address: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v' }}
                />
            )}
            {activeTab === 1 && (
                <StakingPool token={mockToken} />
            )}
            {activeTab === 2 && (
                <LendingPool token={mockToken} />
            )}
        </Box>
    );
};

export default DeFi;
