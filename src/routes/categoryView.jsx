import { Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import Catalog from "../components/catalog";
import { useTranslation } from "react-i18next";

const get_list_of_publications = async (type) => {
    let data = await invoke("catalog_get_list_from_type", {publicationType: type});
    return data;
}

const BibleView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Bible");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const TractView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Tract");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const KingdomMinistryView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Kingdom Ministry");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const IndexesView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Index");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const BookView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Book");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const BrochureView = () => {
    const { t } = useTranslation();
    const [brochurePublications, setBrochurePublications] = useState([]);
    const [bookletPublications, setBookletPublications] = useState([]);


    useEffect(() => {
        const fetchBrochurePublications = async () => { 
            let publication_data = await get_list_of_publications("Brochure");
            setBrochurePublications(publication_data);
        }

        const fetchBookletsPublications = async () => { 
            let publication_data = await get_list_of_publications("Booklet");
            setBookletPublications(publication_data);
        }

        fetchBrochurePublications();
        fetchBookletsPublications();
    }, [])
    return (
        <>
        <Catalog publications={brochurePublications}/>
        <Typography variant="h4">{t("library_categories.publication_types.booklets")}</Typography>
        <Catalog publications={bookletPublications}/>
        </>
    );
}

const WatchtowerView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Watchtower");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const AwakeView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Awake!");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const MeetingWorkbookView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Meeting Workbook");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const ProgramView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Program");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const WebView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Web");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const ManualGuidelinesView = () => {
    const [publications, setPublications] = useState([]);

    useEffect(() => {
        const fetchPublications = async () => { 
            let publication_data = await get_list_of_publications("Manual/Guidelines");
            setPublications(publication_data);
        }

        fetchPublications();
    }, [])
    return (
        <Catalog publications={publications}/>
    );
}

const CategoryView = () => {
    const { category } = useParams();

    switch (category) {
        case "bible":
            return <BibleView />;
        case "kingdom_ministry":
            return <KingdomMinistryView />;
        case "index":
            return <IndexesView />;
        case "tract":
            return <TractView />;
        case "book":
            return <BookView />;
        case "brochure":
            return <BrochureView />;
        case "watchtower":
            return <WatchtowerView />;
        case "awake":
            return <AwakeView />;
        case "meeting_workbook":
            return <MeetingWorkbookView />;
        case "program":
            return <ProgramView />;
        case "web":
            return <WebView />;
        case "manual_guidelines":
            return <ManualGuidelinesView />;
        default:
            return <Typography variant="h1">Category not found</Typography>;
    }
};

export default CategoryView;