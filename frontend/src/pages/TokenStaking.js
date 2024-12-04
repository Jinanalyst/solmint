import React from 'react';
import {
    Box,
    Typography,
    Card,
    CardContent,
    Alert,
} from '@mui/material';
import { Construction } from '@mui/icons-material';

export default function TokenStaking() {
    return (
        <Box>
            <Typography variant="h4" gutterBottom fontWeight="bold">
                Token Staking
            </Typography>

            <Card>
                <CardContent sx={{ textAlign: 'center', py: 8 }}>
                    <Construction sx={{ fontSize: 64, color: 'primary.main', mb: 2 }} />
                    <Typography variant="h5" gutterBottom>
                        Coming Soon
                    </Typography>
                    <Typography color="text.secondary">
                        Our staking platform is under development. Soon you'll be able to stake
                        tokens and earn rewards!
                    </Typography>
                </CardContent>
            </Card>
        </Box>
    );
}
