import React from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import Home from "./routes/home";
import SummaryView from "./routes/summaryView";
import PubViewRedirect from "./routes/pubRedirectView";
import ChapterView from "./routes/chapterView";
import "./main.css"

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/summary/:lang/:category/:pubSymbol", 
    element: <SummaryView />
  },
  {
    path:"/redirect/:lang/:category/:pubSymbol/:chapterId",
    element: <PubViewRedirect />
  },
  {
    path:"/pubview/:lang/:category/:pubSymbol/:chapterId",
    element: <ChapterView />
  }
]);

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <RouterProvider router={router}/>
  </React.StrictMode>,
);
