import React from 'react';
import {
  Box,
  Container,
  Typography,
  Button,
  Grid,
  Card,
  CardContent,
  useTheme,
} from '@mui/material';
import {
  Token as TokenIcon,
  AccountBalance as StakingIcon,
  SwapHoriz as SwapIcon,
  Security as SecurityIcon,
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';

const Feature = ({ icon, title, description }) => {
  const theme = useTheme();
  return (
    <Card sx={{ height: '100%', backgroundColor: 'background.paper' }}>
      <CardContent>
        <Box display="flex" flexDirection="column" alignItems="center" textAlign="center">
          <Box
            sx={{
              backgroundColor: 'primary.main',
              borderRadius: '50%',
              p: 2,
              mb: 2,
              color: 'primary.contrastText',
            }}
          >
            {icon}
          </Box>
          <Typography variant="h5" gutterBottom>
            {title}
          </Typography>
          <Typography variant="body1" color="text.secondary">
            {description}
          </Typography>
        </Box>
      </CardContent>
    </Card>
  );
};

const Landing = () => {
  const navigate = useNavigate();
  const theme = useTheme();

  const features = [
    {
      icon: <TokenIcon fontSize="large" />,
      title: 'Token Creation',
      description: 'Create your own Solana token in minutes with customizable parameters and metadata.',
    },
    {
      icon: <StakingIcon fontSize="large" />,
      title: 'Staking',
      description: 'Earn rewards by staking your tokens with flexible lock periods and competitive APR.',
    },
    {
      icon: <SwapIcon fontSize="large" />,
      title: 'Token Swap',
      description: 'Swap tokens instantly with automated market making and optimal pricing.',
    },
    {
      icon: <SecurityIcon fontSize="large" />,
      title: 'Secure Platform',
      description: 'Built with industry-leading security practices and audited smart contracts.',
    },
  ];

  return (
    <Box>
      {/* Hero Section */}
      <Box
        sx={{
          background: `linear-gradient(45deg, ${theme.palette.primary.dark} 0%, ${theme.palette.secondary.dark} 100%)`,
          pt: 12,
          pb: 6,
        }}
      >
        <Container maxWidth="lg">
          <Grid container spacing={4} alignItems="center">
            <Grid item xs={12} md={6}>
              <Typography
                variant="h1"
                sx={{
                  fontWeight: 700,
                  color: 'common.white',
                  mb: 2,
                }}
              >
                Create and Manage Solana Tokens with Ease
              </Typography>
              <Typography
                variant="h5"
                sx={{
                  color: 'rgba(255, 255, 255, 0.8)',
                  mb: 4,
                }}
              >
                The most comprehensive platform for token creation, staking, and DeFi on Solana
              </Typography>
              <Button
                variant="contained"
                size="large"
                onClick={() => navigate('/create')}
                sx={{
                  mr: 2,
                  backgroundColor: 'common.white',
                  color: 'primary.main',
                  '&:hover': {
                    backgroundColor: 'rgba(255, 255, 255, 0.9)',
                  },
                }}
              >
                Create Token
              </Button>
              <Button
                variant="outlined"
                size="large"
                onClick={() => navigate('/defi')}
                sx={{
                  borderColor: 'common.white',
                  color: 'common.white',
                  '&:hover': {
                    borderColor: 'rgba(255, 255, 255, 0.9)',
                    backgroundColor: 'rgba(255, 255, 255, 0.1)',
                  },
                }}
              >
                Explore DeFi
              </Button>
            </Grid>
          </Grid>
        </Container>
      </Box>

      {/* Features Section */}
      <Container maxWidth="lg" sx={{ py: 8 }}>
        <Typography
          variant="h2"
          align="center"
          sx={{ mb: 6 }}
        >
          Platform Features
        </Typography>
        <Grid container spacing={4}>
          {features.map((feature, index) => (
            <Grid item xs={12} sm={6} md={3} key={index}>
              <Feature {...feature} />
            </Grid>
          ))}
        </Grid>
      </Container>

      {/* CTA Section */}
      <Box sx={{ bgcolor: 'background.paper', py: 8 }}>
        <Container maxWidth="lg">
          <Box textAlign="center">
            <Typography variant="h3" gutterBottom>
              Ready to Create Your Token?
            </Typography>
            <Typography variant="body1" color="text.secondary" sx={{ mb: 4 }}>
              Join thousands of projects building on Solana with Solmint
            </Typography>
            <Button
              variant="contained"
              size="large"
              onClick={() => navigate('/create')}
            >
              Get Started Now
            </Button>
          </Box>
        </Container>
      </Box>
    </Box>
  );
};

export default Landing;
