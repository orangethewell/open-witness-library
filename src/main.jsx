import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import router from "./routes"
import "./main.css"
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';


import { alpha, createTheme, ThemeProvider, } from '@mui/material/styles';
import { CssBaseline } from "@mui/material";
import './i18n'

const theme = createTheme({
  colorSchemes: {
    dark: {
      palette: {
        primary: {
          main: alpha('#D6AFFF', 0.7),
        },
        secondary: {
          main: alpha('#D6AFFF', 0.7),
        },
      },
    },
  },
  palette: {
    primary: {
      main: alpha('#7F00FF', 0.7),
    },
    secondary: {
      main: alpha('#7F00FF', 0.7),
    },
  },
});

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline enableColorScheme />
      <RouterProvider router={router}/>
    </ThemeProvider>
  </React.StrictMode>,
);
