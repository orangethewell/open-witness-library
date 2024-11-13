import Home from "./routes/homeView";
import SettingsView from "./routes/settingsView";
import SummaryView from "./routes/summaryView";
import PubViewRedirect from "./routes/pubRedirectView";
import ChapterView from "./routes/chapterView";
import Root from "./routes/rootView";
import { createBrowserRouter } from "react-router-dom";

const routes = [
    {
        path: "/",
        element: <Root />,
        children: [
            {
                index: true,
                element: <Home />,
            },
            {
                path: "/settings",
                element: <SettingsView />,
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
        ],
    },
] 

const router = createBrowserRouter(routes);

export default router;