import React from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  Box,
  useTheme,
  Button,
} from '@mui/material';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { Link } from 'react-router-dom';
import TokenIcon from '@mui/icons-material/Token';

const Header = () => {
  const theme = useTheme();

  return (
    <AppBar position="static" color="transparent" elevation={1}>
      <Toolbar>
        <Box display="flex" alignItems="center" flexGrow={1}>
          <TokenIcon sx={{ mr: 1, color: theme.palette.primary.main }} />
          <Typography
            variant="h6"
            component={Link}
            to="/"
            sx={{
              textDecoration: 'none',
              color: theme.palette.text.primary,
              fontWeight: 'bold',
              display: 'flex',
              alignItems: 'center',
            }}
          >
            SOLMINT
          </Typography>
          
          <Box ml={4}>
            <Button
              component={Link}
              to="/create"
              color="primary"
              sx={{ mr: 2 }}
            >
              Create Token
            </Button>
            <Button
              component={Link}
              to="/defi"
              color="primary"
              sx={{ mr: 2 }}
            >
              DeFi
            </Button>
            <Button
              component={Link}
              to="/dashboard"
              color="primary"
            >
              Dashboard
            </Button>
          </Box>
        </Box>

        <WalletMultiButton />
      </Toolbar>
    </AppBar>
  );
};

export default Header;
