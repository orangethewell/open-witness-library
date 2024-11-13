import Home from "./routes/homeView";
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
        ],
    },
] 

const router = createBrowserRouter(routes);

export default router;