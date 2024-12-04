import React, { useState, useEffect } from 'react';
import {
    Box,
    Grid,
    Card,
    CardContent,
    Typography,
    TextField,
    Button,
    Stepper,
    Step,
    StepLabel,
    Alert,
    CircularProgress,
    Switch,
    FormControlLabel,
    Tooltip,
    IconButton,
} from '@mui/material';
import {
    ContentCopy,
    Info,
    CheckCircle,
} from '@mui/icons-material';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey, clusterApiUrl, LAMPORTS_PER_SOL } from '@solana/web3.js';
import * as splToken from '@solana/spl-token';
import { toast } from 'react-toastify';

const steps = ['Configure Token', 'Review Details', 'Create Token'];

const FEE_STRUCTURE = {
    BASE_FEE: 0.1,          // 0.1 SOL
    MINT_AUTHORITY_FEE: 0.05, // 0.05 SOL
    FREEZE_AUTHORITY_FEE: 0.05, // 0.05 SOL
};

const calculateTotalFee = (enableMint, enableFreeze) => {
    let totalFee = FEE_STRUCTURE.BASE_FEE;
    if (enableMint) totalFee += FEE_STRUCTURE.MINT_AUTHORITY_FEE;
    if (enableFreeze) totalFee += FEE_STRUCTURE.FREEZE_AUTHORITY_FEE;
    return totalFee;
};

export default function CreateToken() {
    const { publicKey, signTransaction } = useWallet();
    const [activeStep, setActiveStep] = useState(0);
    const [loading, setLoading] = useState(false);
    const [network, setNetwork] = useState('devnet');
    const [tokenData, setTokenData] = useState({
        name: '',
        symbol: '',
        totalSupply: '',
        decimals: '9',
        description: '',
        website: '',
        twitter: '',
        discord: '',
        enableMint: false,
        enableFreeze: false
    });
    const [mintAddress, setMintAddress] = useState('');
    const [totalFee, setTotalFee] = useState(FEE_STRUCTURE.BASE_FEE);

    const handleNetworkChange = (event) => {
        setNetwork(event.target.checked ? 'mainnet-beta' : 'devnet');
    };

    const handleChange = (e) => {
        setTokenData({
            ...tokenData,
            [e.target.name]: e.target.type === 'checkbox' ? e.target.checked : e.target.value,
        });
        if (e.target.name === 'enableMint' || e.target.name === 'enableFreeze') {
            setTotalFee(calculateTotalFee(tokenData.enableMint, tokenData.enableFreeze));
        }
    };

    const handleCopy = (text) => {
        navigator.clipboard.writeText(text);
        toast.success('Copied to clipboard!');
    };

    const validateStep = () => {
        if (activeStep === 0) {
            if (!tokenData.name || !tokenData.symbol || !tokenData.totalSupply) {
                toast.error('Please fill in all required fields');
                return false;
            }
            if (parseInt(tokenData.totalSupply) <= 0) {
                toast.error('Total supply must be greater than 0');
                return false;
            }
        }
        return true;
    };

    const handleNext = () => {
        if (validateStep()) {
            setActiveStep((prev) => prev + 1);
        }
    };

    const handleBack = () => {
        setActiveStep((prev) => prev - 1);
    };

    const createToken = async () => {
        if (!publicKey) {
            toast.error('Please connect your wallet first');
            return;
        }

        try {
            setLoading(true);
            const connection = new Connection(clusterApiUrl(network), 'confirmed');
            const balance = await connection.getBalance(publicKey);
            const requiredBalance = totalFee * LAMPORTS_PER_SOL;

            if (balance < requiredBalance) {
                toast.error(`Insufficient balance. You need at least ${totalFee} SOL for fees`);
                return;
            }

            const mint = await splToken.createMint(
                connection,
                {
                    publicKey,
                    signTransaction,
                },
                publicKey,
                publicKey,
                parseInt(tokenData.decimals)
            );

            setMintAddress(mint.toString());
            toast.success('Token created successfully!');
            setActiveStep(3);
        } catch (error) {
            console.error(error);
            toast.error('Error creating token: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    const renderStepContent = () => {
        switch (activeStep) {
            case 0:
                return (
                    <Grid container spacing={3}>
                        <Grid item xs={12}>
                            <TextField
                                fullWidth
                                label="Token Name"
                                name="name"
                                value={tokenData.name}
                                onChange={handleChange}
                                required
                                helperText="The name of your token (e.g., 'Solana')"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <TextField
                                fullWidth
                                label="Token Symbol"
                                name="symbol"
                                value={tokenData.symbol}
                                onChange={handleChange}
                                required
                                helperText="The symbol of your token (e.g., 'SOL')"
                            />
                        </Grid>
                        <Grid item xs={12} md={6}>
                            <TextField
                                fullWidth
                                label="Total Supply"
                                name="totalSupply"
                                type="number"
                                value={tokenData.totalSupply}
                                onChange={handleChange}
                                required
                                helperText="The total supply of your token"
                            />
                        </Grid>
                        <Grid item xs={12} md={6}>
                            <TextField
                                fullWidth
                                label="Decimals"
                                name="decimals"
                                type="number"
                                value={tokenData.decimals}
                                onChange={handleChange}
                                required
                                helperText="Default is 9 for Solana tokens"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <TextField
                                fullWidth
                                label="Description"
                                name="description"
                                value={tokenData.description}
                                onChange={handleChange}
                                multiline
                                rows={3}
                                helperText="A brief description of your token"
                            />
                        </Grid>
                        <Grid item xs={12} md={4}>
                            <TextField
                                fullWidth
                                label="Website"
                                name="website"
                                value={tokenData.website}
                                onChange={handleChange}
                                helperText="Your project's website"
                            />
                        </Grid>
                        <Grid item xs={12} md={4}>
                            <TextField
                                fullWidth
                                label="Twitter"
                                name="twitter"
                                value={tokenData.twitter}
                                onChange={handleChange}
                                helperText="Your project's Twitter handle"
                            />
                        </Grid>
                        <Grid item xs={12} md={4}>
                            <TextField
                                fullWidth
                                label="Discord"
                                name="discord"
                                value={tokenData.discord}
                                onChange={handleChange}
                                helperText="Your project's Discord server"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <FormControlLabel
                                control={
                                    <Switch
                                        checked={tokenData.enableMint}
                                        onChange={handleChange}
                                        name="enableMint"
                                        color="primary"
                                    />
                                }
                                label="Enable Mint Authority"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <FormControlLabel
                                control={
                                    <Switch
                                        checked={tokenData.enableFreeze}
                                        onChange={handleChange}
                                        name="enableFreeze"
                                        color="primary"
                                    />
                                }
                                label="Enable Freeze Authority"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <Typography variant="body1">Total Fee: {totalFee} SOL</Typography>
                        </Grid>
                    </Grid>
                );
            case 1:
                return (
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Review Token Details
                            </Typography>
                            <Grid container spacing={2}>
                                {Object.entries(tokenData).map(([key, value]) => (
                                    value && (
                                        <Grid item xs={12} key={key}>
                                            <Typography variant="subtitle2" color="text.secondary">
                                                {key.charAt(0).toUpperCase() + key.slice(1)}
                                            </Typography>
                                            <Typography variant="body1">{value}</Typography>
                                        </Grid>
                                    )
                                ))}
                            </Grid>
                        </CardContent>
                    </Card>
                );
            case 2:
                return (
                    <Box textAlign="center">
                        <Typography variant="h6" gutterBottom>
                            Create Your Token
                        </Typography>
                        <Typography color="text.secondary" paragraph>
                            Click the button below to create your token on the {network} network.
                        </Typography>
                        <Button
                            variant="contained"
                            color="primary"
                            onClick={createToken}
                            disabled={loading}
                            size="large"
                        >
                            {loading ? <CircularProgress size={24} /> : 'Create Token'}
                        </Button>
                    </Box>
                );
            default:
                return null;
        }
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom fontWeight="bold">
                Create Token
            </Typography>

            <Card sx={{ mb: 3 }}>
                <CardContent>
                    <FormControlLabel
                        control={
                            <Switch
                                checked={network === 'mainnet-beta'}
                                onChange={handleNetworkChange}
                                color="primary"
                            />
                        }
                        label={network === 'mainnet-beta' ? 'Mainnet' : 'Devnet'}
                    />
                </CardContent>
            </Card>

            <Stepper activeStep={activeStep} sx={{ mb: 4 }}>
                {steps.map((label) => (
                    <Step key={label}>
                        <StepLabel>{label}</StepLabel>
                    </Step>
                ))}
            </Stepper>

            <Card>
                <CardContent>
                    {renderStepContent()}

                    {mintAddress && (
                        <Alert
                            severity="success"
                            sx={{ mt: 3 }}
                            action={
                                <IconButton
                                    size="small"
                                    onClick={() => handleCopy(mintAddress)}
                                >
                                    <ContentCopy fontSize="small" />
                                </IconButton>
                            }
                        >
                            Token created! Mint address: {mintAddress.slice(0, 8)}...{mintAddress.slice(-8)}
                        </Alert>
                    )}

                    {activeStep !== 3 && (
                        <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 3 }}>
                            {activeStep !== 0 && (
                                <Button onClick={handleBack} sx={{ mr: 1 }}>
                                    Back
                                </Button>
                            )}
                            {activeStep < 2 && (
                                <Button
                                    variant="contained"
                                    onClick={handleNext}
                                    color="primary"
                                >
                                    Next
                                </Button>
                            )}
                        </Box>
                    )}
                </CardContent>
            </Card>
        </Box>
    );
}
