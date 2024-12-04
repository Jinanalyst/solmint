import React from 'react';
import {
    Box,
    Typography,
    Card,
    CardContent,
    Alert,
} from '@mui/material';
import { Construction } from '@mui/icons-material';

export default function TokenSwap() {
    return (
        <Box>
            <Typography variant="h4" gutterBottom fontWeight="bold">
                Token Swap
            </Typography>

            <Card>
                <CardContent sx={{ textAlign: 'center', py: 8 }}>
                    <Construction sx={{ fontSize: 64, color: 'primary.main', mb: 2 }} />
                    <Typography variant="h5" gutterBottom>
                        Coming Soon
                    </Typography>
                    <Typography color="text.secondary">
                        We're working hard to bring you token swapping functionality.
                        Stay tuned for updates!
                    </Typography>
                </CardContent>
            </Card>
        </Box>
    );
}
