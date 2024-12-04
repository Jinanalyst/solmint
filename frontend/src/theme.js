import { createTheme } from '@mui/material/styles';

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#14F195', // Solana Green
      light: '#45f5ac',
      dark: '#0ea867',
      contrastText: '#000000',
    },
    secondary: {
      main: '#9945FF', // Solana Purple
      light: '#b06fff',
      dark: '#7a37cc',
      contrastText: '#ffffff',
    },
    background: {
      default: '#0F1624',
      paper: '#1A2332',
    },
    text: {
      primary: '#FFFFFF',
      secondary: '#A3A3A3',
    },
  },
  typography: {
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
    h1: {
      fontSize: '2.5rem',
      fontWeight: 600,
      marginBottom: '1rem',
    },
    h2: {
      fontSize: '2rem',
      fontWeight: 500,
      marginBottom: '0.875rem',
    },
    h3: {
      fontSize: '1.75rem',
      fontWeight: 500,
      marginBottom: '0.75rem',
    },
    h4: {
      fontSize: '1.5rem',
      fontWeight: 500,
      marginBottom: '0.625rem',
    },
    h5: {
      fontSize: '1.25rem',
      fontWeight: 500,
      marginBottom: '0.5rem',
    },
    h6: {
      fontSize: '1rem',
      fontWeight: 500,
      marginBottom: '0.375rem',
    },
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: '8px',
          textTransform: 'none',
          fontWeight: 500,
          padding: '8px 24px',
        },
        contained: {
          boxShadow: 'none',
          '&:hover': {
            boxShadow: 'none',
          },
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: '16px',
          boxShadow: '0 4px 20px rgba(0, 0, 0, 0.15)',
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiOutlinedInput-root': {
            borderRadius: '8px',
          },
        },
      },
    },
    MuiAppBar: {
      styleOverrides: {
        root: {
          backgroundColor: 'rgba(26, 35, 50, 0.8)',
          backdropFilter: 'blur(20px)',
        },
      },
    },
    MuiDrawer: {
      styleOverrides: {
        paper: {
          backgroundColor: '#1A2332',
        },
      },
    },
    MuiCssBaseline: {
      styleOverrides: {
        body: {
          scrollbarColor: "#6b6b6b #2b2b2b",
          "&::-webkit-scrollbar, & *::-webkit-scrollbar": {
            backgroundColor: "#2b2b2b",
            width: '8px',
          },
          "&::-webkit-scrollbar-thumb, & *::-webkit-scrollbar-thumb": {
            borderRadius: 8,
            backgroundColor: "#6b6b6b",
            minHeight: 24,
          },
          "&::-webkit-scrollbar-thumb:focus, & *::-webkit-scrollbar-thumb:focus": {
            backgroundColor: "#959595",
          },
          "&::-webkit-scrollbar-thumb:active, & *::-webkit-scrollbar-thumb:active": {
            backgroundColor: "#959595",
          },
          "&::-webkit-scrollbar-thumb:hover, & *::-webkit-scrollbar-thumb:hover": {
            backgroundColor: "#959595",
          },
        },
      },
    },
  },
});

export default theme;
