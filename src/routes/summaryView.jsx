import React, { useState, useEffect, useMemo } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import PubSummary from '../components/summary';
import { invoke } from '@tauri-apps/api/core';

const SummaryView = () => {
    const { lang, category, pubSymbol } = useParams();  // Utiliza a hook useParams para pegar os parâmetros da URL
    const [chapterList, setChapterList] = useState([]);
    const navigate = useNavigate();

    // Carrega os capítulos quando os parâmetros mudam
    useEffect(() => {
        const fetchData = async () => {
            let chapters = await invoke("pubcatalog_get_summary_from", {lang, category, pubSymbol: pubSymbol});
            setChapterList(chapters.arrayof);
        };
        fetchData();
    }, [lang, category, pubSymbol]);

    const output = useMemo(() => {
        return <PubSummary chapters={chapterList} lang={lang} category={category} pubSymbol={pubSymbol} />;
    }, [chapterList]);

    return <>{output}</>;
};

export default SummaryView;
