import React from 'react';
import {
    Box,
    Grid,
    Card,
    CardContent,
    Typography,
    Button,
    useTheme,
} from '@mui/material';
import { useNavigate } from 'react-router-dom';
import {
    Token,
    SwapHoriz,
    AccountBalance,
    ArrowForward,
} from '@mui/icons-material';

const features = [
    {
        title: 'Create Token',
        description: 'Create your own Solana token in minutes',
        icon: <Token sx={{ fontSize: 40 }} />,
        path: '/create-token',
        color: '#9945FF',
    },
    {
        title: 'Token Swap',
        description: 'Swap tokens instantly with low fees',
        icon: <SwapHoriz sx={{ fontSize: 40 }} />,
        path: '/token-swap',
        color: '#14F195',
    },
    {
        title: 'Token Staking',
        description: 'Stake tokens and earn rewards',
        icon: <AccountBalance sx={{ fontSize: 40 }} />,
        path: '/token-staking',
        color: '#00C2FF',
    },
];

export default function Dashboard() {
    const navigate = useNavigate();
    const theme = useTheme();

    return (
        <Box>
            <Typography variant="h4" gutterBottom fontWeight="bold">
                Welcome to Solmint
            </Typography>
            <Typography variant="subtitle1" color="text.secondary" gutterBottom>
                Your all-in-one platform for Solana token operations
            </Typography>

            <Grid container spacing={3} sx={{ mt: 2 }}>
                {features.map((feature) => (
                    <Grid item xs={12} md={4} key={feature.title}>
                        <Card
                            sx={{
                                height: '100%',
                                display: 'flex',
                                flexDirection: 'column',
                                '&:hover': {
                                    transform: 'translateY(-4px)',
                                    transition: 'transform 0.2s ease-in-out',
                                },
                            }}
                        >
                            <CardContent sx={{ flexGrow: 1 }}>
                                <Box
                                    sx={{
                                        display: 'flex',
                                        alignItems: 'center',
                                        mb: 2,
                                    }}
                                >
                                    <Box
                                        sx={{
                                            p: 1,
                                            borderRadius: 2,
                                            bgcolor: feature.color + '20',
                                            color: feature.color,
                                        }}
                                    >
                                        {feature.icon}
                                    </Box>
                                </Box>
                                <Typography variant="h6" gutterBottom>
                                    {feature.title}
                                </Typography>
                                <Typography
                                    variant="body2"
                                    color="text.secondary"
                                    sx={{ mb: 2 }}
                                >
                                    {feature.description}
                                </Typography>
                                <Button
                                    variant="outlined"
                                    endIcon={<ArrowForward />}
                                    onClick={() => navigate(feature.path)}
                                    sx={{
                                        borderColor: feature.color,
                                        color: feature.color,
                                        '&:hover': {
                                            borderColor: feature.color,
                                            bgcolor: feature.color + '10',
                                        },
                                    }}
                                >
                                    Get Started
                                </Button>
                            </CardContent>
                        </Card>
                    </Grid>
                ))}
            </Grid>
        </Box>
    );
}
