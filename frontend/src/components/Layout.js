import React, { useState } from 'react';
import {
    AppBar,
    Box,
    Toolbar,
    IconButton,
    Typography,
    Drawer,
    List,
    ListItem,
    ListItemIcon,
    ListItemText,
    useTheme,
    Button,
} from '@mui/material';
import {
    Menu as MenuIcon,
    Token,
    SwapHoriz,
    AccountBalance,
    Dashboard,
    Close,
} from '@mui/icons-material';
import { useNavigate, useLocation } from 'react-router-dom';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

const drawerWidth = 240;

const menuItems = [
    { text: 'Dashboard', icon: <Dashboard />, path: '/' },
    { text: 'Create Token', icon: <Token />, path: '/create-token' },
    { text: 'Token Swap', icon: <SwapHoriz />, path: '/token-swap' },
    { text: 'Token Staking', icon: <AccountBalance />, path: '/token-staking' },
];

export default function Layout({ children }) {
    const [mobileOpen, setMobileOpen] = useState(false);
    const theme = useTheme();
    const navigate = useNavigate();
    const location = useLocation();

    const handleDrawerToggle = () => {
        setMobileOpen(!mobileOpen);
    };

    const drawer = (
        <Box sx={{ bgcolor: 'background.default', height: '100%' }}>
            <Box
                sx={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    p: 2,
                }}
            >
                <Typography variant="h6" component="div" sx={{ color: 'primary.main' }}>
                    Solmint
                </Typography>
                <IconButton
                    color="inherit"
                    aria-label="close drawer"
                    edge="start"
                    onClick={handleDrawerToggle}
                    sx={{ display: { sm: 'none' } }}
                >
                    <Close />
                </IconButton>
            </Box>
            <List>
                {menuItems.map((item) => (
                    <ListItem
                        button
                        key={item.text}
                        onClick={() => {
                            navigate(item.path);
                            setMobileOpen(false);
                        }}
                        sx={{
                            bgcolor: location.pathname === item.path ? 'action.selected' : 'transparent',
                            '&:hover': {
                                bgcolor: 'action.hover',
                            },
                        }}
                    >
                        <ListItemIcon sx={{ color: 'primary.main' }}>{item.icon}</ListItemIcon>
                        <ListItemText primary={item.text} />
                    </ListItem>
                ))}
            </List>
        </Box>
    );

    return (
        <Box sx={{ display: 'flex', minHeight: '100vh' }}>
            <AppBar
                position="fixed"
                sx={{
                    width: { sm: `calc(100% - ${drawerWidth}px)` },
                    ml: { sm: `${drawerWidth}px` },
                    bgcolor: 'background.paper',
                    borderBottom: `1px solid ${theme.palette.divider}`,
                }}
                elevation={0}
            >
                <Toolbar>
                    <IconButton
                        color="inherit"
                        aria-label="open drawer"
                        edge="start"
                        onClick={handleDrawerToggle}
                        sx={{ mr: 2, display: { sm: 'none' } }}
                    >
                        <MenuIcon />
                    </IconButton>
                    <Box sx={{ flexGrow: 1 }} />
                    <WalletMultiButton />
                </Toolbar>
            </AppBar>

            <Box
                component="nav"
                sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
            >
                <Drawer
                    variant="temporary"
                    open={mobileOpen}
                    onClose={handleDrawerToggle}
                    ModalProps={{
                        keepMounted: true,
                    }}
                    sx={{
                        display: { xs: 'block', sm: 'none' },
                        '& .MuiDrawer-paper': {
                            boxSizing: 'border-box',
                            width: drawerWidth,
                            bgcolor: 'background.default',
                        },
                    }}
                >
                    {drawer}
                </Drawer>
                <Drawer
                    variant="permanent"
                    sx={{
                        display: { xs: 'none', sm: 'block' },
                        '& .MuiDrawer-paper': {
                            boxSizing: 'border-box',
                            width: drawerWidth,
                            bgcolor: 'background.default',
                            borderRight: `1px solid ${theme.palette.divider}`,
                        },
                    }}
                    open
                >
                    {drawer}
                </Drawer>
            </Box>

            <Box
                component="main"
                sx={{
                    flexGrow: 1,
                    p: 3,
                    width: { sm: `calc(100% - ${drawerWidth}px)` },
                    mt: 8,
                }}
            >
                {children}
            </Box>
        </Box>
    );
}
