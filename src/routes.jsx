import Home from "./routes/homeView";
import SettingsView from "./routes/settingsView";
import SummaryView from "./routes/summaryView";
import ChapterView from "./routes/chapterView";
import Root from "./routes/rootView";
import { createBrowserRouter } from "react-router-dom";
import LibraryView from "./routes/libraryView";
import CategoryView from "./routes/categoryView";

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
                path: "/library",
                element: <LibraryView />,
            },
            {
                path: "/library/:category",
                element: <CategoryView />,
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
                path:"/pubview/:lang/:category/:pubSymbol/:chapterId",
                element: <ChapterView />
            }
        ],
    },
] 

const router = createBrowserRouter(routes);

export default router;