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
    Tabs,
    Tab,
    Alert,
} from '@mui/material';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import { AccountBalance, LocalAtm, Calculate } from '@mui/icons-material';

const LendingPool = ({ token }) => {
    const { publicKey, signTransaction } = useWallet();
    const [loading, setLoading] = useState(false);
    const [activeTab, setActiveTab] = useState(0);
    const [lendingData, setLendingData] = useState({
        depositAmount: '',
        borrowAmount: '',
        depositedBalance: '0',
        borrowedBalance: '0',
        availableToBorrow: '0',
        collateralRatio: '150', // 150%
    });

    const [poolStats, setPoolStats] = useState({
        totalDeposits: '0',
        totalBorrows: '0',
        depositAPY: '5.2',
        borrowAPR: '7.8',
        utilizationRate: '65',
    });

    const handleTabChange = (event, newValue) => {
        setActiveTab(newValue);
    };

    const handleInputChange = (event) => {
        const { name, value } = event.target;
        setLendingData(prev => ({
            ...prev,
            [name]: value
        }));
    };

    const deposit = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement deposit logic here
            // 1. Create transaction
            // 2. Add token transfer instruction
            // 3. Add deposit instruction
            // 4. Sign and send transaction
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Tokens deposited successfully!');
            setLendingData(prev => ({
                ...prev,
                depositAmount: ''
            }));
        } catch (error) {
            console.error('Error depositing tokens:', error);
            alert('Failed to deposit tokens: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const withdraw = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement withdrawal logic here
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Tokens withdrawn successfully!');
        } catch (error) {
            console.error('Error withdrawing tokens:', error);
            alert('Failed to withdraw tokens: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const borrow = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement borrowing logic here
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Tokens borrowed successfully!');
            setLendingData(prev => ({
                ...prev,
                borrowAmount: ''
            }));
        } catch (error) {
            console.error('Error borrowing tokens:', error);
            alert('Failed to borrow tokens: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const repay = async () => {
        if (!publicKey) {
            alert('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            // Implement repayment logic here
            
            // Placeholder for actual implementation
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            alert('Tokens repaid successfully!');
        } catch (error) {
            console.error('Error repaying tokens:', error);
            alert('Failed to repay tokens: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        // Fetch lending pool data when component mounts
        const fetchLendingData = async () => {
            try {
                // Implement lending data fetching logic here
                // Update lendingData and poolStats states
            } catch (error) {
                console.error('Error fetching lending data:', error);
            }
        };

        fetchLendingData();
    }, [publicKey]);

    const renderDepositTab = () => (
        <Box sx={{ mt: 2 }}>
            <Grid container spacing={2}>
                <Grid item xs={12}>
                    <TextField
                        fullWidth
                        label="Deposit Amount"
                        name="depositAmount"
                        value={lendingData.depositAmount}
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
                        onClick={deposit}
                        disabled={loading || !lendingData.depositAmount}
                    >
                        {loading ? <CircularProgress size={24} /> : 'Deposit'}
                    </Button>
                </Grid>
            </Grid>

            <Box sx={{ mt: 4 }}>
                <Typography variant="h6" gutterBottom>
                    Your Deposits
                </Typography>
                <TableContainer component={Paper} variant="outlined">
                    <Table>
                        <TableBody>
                            <TableRow>
                                <TableCell>Deposited Balance</TableCell>
                                <TableCell align="right">
                                    {lendingData.depositedBalance} {token?.symbol}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Earned Interest</TableCell>
                                <TableCell align="right">
                                    {(parseFloat(lendingData.depositedBalance) * 0.052).toFixed(2)} {token?.symbol}
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </TableContainer>

                <Button
                    variant="outlined"
                    onClick={withdraw}
                    disabled={loading || lendingData.depositedBalance === '0'}
                    fullWidth
                    sx={{ mt: 2 }}
                >
                    Withdraw
                </Button>
            </Box>
        </Box>
    );

    const renderBorrowTab = () => (
        <Box sx={{ mt: 2 }}>
            <Alert severity="info" sx={{ mb: 2 }}>
                Collateral Ratio: {lendingData.collateralRatio}% (Min. Required: 150%)
            </Alert>

            <Grid container spacing={2}>
                <Grid item xs={12}>
                    <TextField
                        fullWidth
                        label="Borrow Amount"
                        name="borrowAmount"
                        value={lendingData.borrowAmount}
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
                        onClick={borrow}
                        disabled={loading || !lendingData.borrowAmount}
                    >
                        {loading ? <CircularProgress size={24} /> : 'Borrow'}
                    </Button>
                </Grid>
            </Grid>

            <Box sx={{ mt: 4 }}>
                <Typography variant="h6" gutterBottom>
                    Your Borrows
                </Typography>
                <TableContainer component={Paper} variant="outlined">
                    <Table>
                        <TableBody>
                            <TableRow>
                                <TableCell>Borrowed Balance</TableCell>
                                <TableCell align="right">
                                    {lendingData.borrowedBalance} {token?.symbol}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Available to Borrow</TableCell>
                                <TableCell align="right">
                                    {lendingData.availableToBorrow} {token?.symbol}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Interest Accrued</TableCell>
                                <TableCell align="right">
                                    {(parseFloat(lendingData.borrowedBalance) * 0.078).toFixed(2)} {token?.symbol}
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </TableContainer>

                <Button
                    variant="outlined"
                    onClick={repay}
                    disabled={loading || lendingData.borrowedBalance === '0'}
                    fullWidth
                    sx={{ mt: 2 }}
                >
                    Repay
                </Button>
            </Box>
        </Box>
    );

    return (
        <Box>
            <Typography variant="h5" gutterBottom sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                <LocalAtm color="primary" />
                Lending Pool
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12} md={8}>
                    <Card>
                        <CardContent>
                            <Tabs value={activeTab} onChange={handleTabChange} sx={{ mb: 2 }}>
                                <Tab label="Deposit" />
                                <Tab label="Borrow" />
                            </Tabs>

                            {activeTab === 0 ? renderDepositTab() : renderBorrowTab()}
                        </CardContent>
                    </Card>
                </Grid>

                <Grid item xs={12} md={4}>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Pool Stats
                            </Typography>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Total Deposits
                                </Typography>
                                <Typography variant="h6">
                                    {poolStats.totalDeposits} {token?.symbol}
                                </Typography>
                            </Box>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Total Borrows
                                </Typography>
                                <Typography variant="h6">
                                    {poolStats.totalBorrows} {token?.symbol}
                                </Typography>
                            </Box>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Deposit APY
                                </Typography>
                                <Typography variant="h6">
                                    {poolStats.depositAPY}%
                                </Typography>
                            </Box>
                            <Box sx={{ mb: 2 }}>
                                <Typography color="text.secondary" gutterBottom>
                                    Borrow APR
                                </Typography>
                                <Typography variant="h6">
                                    {poolStats.borrowAPR}%
                                </Typography>
                            </Box>
                            <Box>
                                <Typography color="text.secondary" gutterBottom>
                                    Utilization Rate
                                </Typography>
                                <Typography variant="h6">
                                    {poolStats.utilizationRate}%
                                </Typography>
                            </Box>
                        </CardContent>
                    </Card>
                </Grid>
            </Grid>
        </Box>
    );
};

export default LendingPool;
