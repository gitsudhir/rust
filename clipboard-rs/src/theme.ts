import { createTheme } from "@mui/material/styles";

const createAppTheme = (mode: 'light' | 'dark' = 'light') => createTheme({
  palette: {
    mode,
    primary: {
      main: "#1976d2",
    },
    secondary: {
      main: "#e57373",
    },
    background: {
      default: mode === 'dark' ? '#121212' : "#f5f5f5",
      paper: mode === 'dark' ? '#1e1e1e' : "#ffffff",
    },
  },
  typography: {
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
    h5: {
      fontWeight: 600,
    },
    h6: {
      fontWeight: 600,
    },
  },
  components: {
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 0,
          boxShadow: "0 4px 20px rgba(0,0,0,0.08)",
        },
      },
    },
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          textTransform: "none",
          fontWeight: 500,
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          "& .MuiOutlinedInput-root": {
            borderRadius: 8,
          },
        },
      },
    },
  },
});

export default createAppTheme;