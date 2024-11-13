import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import router from "./routes"
import "./main.css"

import { createTheme, ThemeProvider } from '@mui/material/styles';

const themeOptions = createTheme({
  colorSchemes: {
    light: {
      primary: '#643ed8',
      secondary: '#f50000',
    },
    dark: {
      primary: '#643ed8',
      secondary: '#f50000',
    },
    contrastThreshold: 3,
    tonalOffset: 0.2,
  },
  palette: {
    primary: {
      main: '#643ed8',
    },
    secondary: {
      main: '#f50000',
    },
  },
});

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <ThemeProvider theme={themeOptions}>
      <RouterProvider router={router}/>
    </ThemeProvider>
  </React.StrictMode>,
);
