import React from "react";
import ReactDOM from "react-dom/client";
import {
  RouterProvider,
} from "react-router-dom";
import router from "./routes"
import "./main.css"
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';


import { createTheme, StyledEngineProvider, ThemeProvider } from '@mui/material/styles';
import { CssBaseline } from "@mui/material";
import './i18n'

const theme = createTheme({
  cssVariables: true,
  colorSchemes: {
    dark: {
      palette: {
        primary: {
          main: '#D6AFFF',
        },
        secondary: {
          main: '#D6AFFF',
        },
        stackButton: {
          main: "#FFFFFF22",
        }
      },
    },
    light: {
      palette: {
        primary: {
          main: "#9f2dc4",
        },
        secondary: {
          main: "#9f2dc4",
        },
        stackButton: {
          main: "#FFF",
        }
      },
    }
  }
});

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <StyledEngineProvider injectFirst>
      <ThemeProvider theme={theme}>
        <CssBaseline enableColorScheme />
        <RouterProvider router={router}/>
      </ThemeProvider>
    </StyledEngineProvider>
  </React.StrictMode>,
);
