import Home from "./routes/homeView";
import SettingsView from "./routes/settingsView";
import Root from "./routes/rootView";
import { createBrowserRouter } from "react-router-dom";
import LibraryView from "./routes/libraryView";
import CategoryView from "./routes/categoryView";
import PublicationView from "./routes/publicationView";
import DocumentView from "./routes/documentView";

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
                path: "/publication/:symbol",
                element: <PublicationView />,
            },
            {
                path: "/publication/:symbol/:documentId",
                element: <DocumentView />,
            },
            {
                path: "/settings",
                element: <SettingsView />,
            },
        ],
    },
] 

const router = createBrowserRouter(routes);

export default router;