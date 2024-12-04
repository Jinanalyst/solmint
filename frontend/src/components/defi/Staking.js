import React, { useState, useEffect } from 'react';
import {
    Box,
    Card,
    CardContent,
    Typography,
    TextField,
    Button,
    Grid,
    CircularProgress,
    InputAdornment,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    Paper,
} from '@mui/material';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import { AccountBalance, Timer } from '@mui/icons-material';

const StakingPool = ({ token }) => {
    const { publicKey, signTransaction } = useWallet();
    const [loading, setLoading] = useState(false);
    const [stakingData, setStakingData] = useState({
        stakeAmount: '',
        stakedBalance: '0',
        rewardBalance: '0',
        totalStaked: '0',
        stakingPeriod: '30', // days
    });

    const [stakingStats, setStakingStats] = useState({
        apr: '12',
        totalStakers: '0',
        totalRewardsDistributed: '0',
    });

    const handleInputChange = (event) => {
        const { name, value } = event.target;
        setStakingData(prev => ({
            ...prev,
            [name]: value
        }));
    };

    const stake = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement staking logic here
            // 1. Create transaction
            // 2. Add token transfer instruction
            // 3. Add staking instruction
            // 4. Sign and send transaction
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Tokens staked successfully!');
            setStakingData(prev => ({
                ...prev,
                stakeAmount: ''
            }));
        } catch (error) {
            console.error('Error staking tokens:', error);
            alert('Failed to stake tokens: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const unstake = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement unstaking logic here
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Tokens unstaked successfully!');
        } catch (error) {
            console.error('Error unstaking tokens:', error);
            alert('Failed to unstake tokens: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const claimRewards = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement reward claiming logic here
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Rewards claimed successfully!');
        } catch (error) {
            console.error('Error claiming rewards:', error);
            alert('Failed to claim rewards: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        // Fetch staking data when component mounts
        const fetchStakingData = async () => {
            try {
                // Implement staking data fetching logic here
                // Update stakingData and stakingStats states
            } catch (error) {
                console.error('Error fetching staking data:', error);
            }
        };

        fetchStakingData();
    }, [publicKey]);

    return (
        <Box>
            <Typography variant="h5" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <AccountBalance color="primary" />
                Token Staking
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12} md={8}>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Stake Tokens
                            </Typography>
                            <Grid container spacing={2}>
                                <Grid item xs={12}>
                                    <TextField
                                        fullWidth
                                        label="Stake Amount"
                                        name="stakeAmount"
                                        value={stakingData.stakeAmount}
                                        onChange={handleInputChange}
                                        type="number"
                                        InputProps={{
                                            endAdornment: <InputAdornment position="end">{token?.symbol}</InputAdornment>
                                        }}
                                    />
                                </Grid>
                                <Grid item xs={12}>
                                    <Button
                                        fullWidth
                                        variant="contained"
                                        onClick={stake}
                                        disabled={loading || !stakingData.stakeAmount}
                                    >
                                        {loading ? <CircularProgress size={24} /> : 'Stake Tokens'}
                                    </Button>
                                </Grid>
                            </Grid>

                            <Box sx={{ mt: 4 }}>
                                <Typography variant="h6" gutterBottom>
                                    Your Staking Position
                                </Typography>
                                <TableContainer component={Paper} variant="outlined">
                                    <Table>
                                        <TableBody>
                                            <TableRow>
                                                <TableCell>Staked Balance</TableCell>
                                                <TableCell align="right">
                                                    {stakingData.stakedBalance} {token?.symbol}
                                                </TableCell>
                                            </TableRow>
                                            <TableRow>
                                                <TableCell>Pending Rewards</TableCell>
                                                <TableCell align="right">
                                                    {stakingData.rewardBalance} {token?.symbol}
                                                </TableCell>
                                            </TableRow>
                                            <TableRow>
                                                <TableCell>Staking Period</TableCell>
                                                <TableCell align="right">
                                                    {stakingData.stakingPeriod} days
                                                </TableCell>
                                            </TableRow>
                                        </TableBody>
                                    </Table>
                                </TableContainer>

                                <Box sx={{ mt: 2, display: 'flex', gap: 2 }}>
                                    <Button
                                        variant="outlined"
                                        onClick={unstake}
                                        disabled={loading || stakingData.stakedBalance === '0'}
                                        fullWidth
                                    >
                                        Unstake
                                    </Button>
                                    <Button
                                        variant="outlined"
                                        color="secondary"
                                        onClick={claimRewards}
                                        disabled={loading || stakingData.rewardBalance === '0'}
                                        fullWidth
                                    >
                                        Claim Rewards
                                    </Button>
                                </Box>
                            </Box>
                        </CardContent>
                    </Card>
                </Grid>

                <Grid item xs={12} md={4}>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Staking Stats
                            </Typography>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Annual Percentage Rate (APR)
                                </Typography>
                                <Typography variant="h6">
                                    {stakingStats.apr}%
                                </Typography>
                            </Box>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Total Value Staked
                                </Typography>
                                <Typography variant="h6">
                                    {stakingData.totalStaked} {token?.symbol}
                                </Typography>
                            </Box>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Total Stakers
                                </Typography>
                                <Typography variant="h6">
                                    {stakingStats.totalStakers}
                                </Typography>
                            </Box>
                            <Box>
                                <Typography color="text.secondary" gutterBottom>
                                    Total Rewards Distributed
                                </Typography>
                                <Typography variant="h6">
                                    {stakingStats.totalRewardsDistributed} {token?.symbol}
                                </Typography>
                            </Box>
                        </CardContent>
                    </Card>
                </Grid>
            </Grid>
        </Box>
    );
};

export default StakingPool;
