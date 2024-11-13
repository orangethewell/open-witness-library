import React, { useState } from 'react';
import PubCatalog from '../components/catalog';
import { invoke } from '@tauri-apps/api/core';

const Home = () => {
    const [publications, setPublications] = useState([]);
    
    const handleClick = async () => {
        let publication_data = await invoke("pubcatalog_get_list_from", {lang: 'T', category: 'bk', startIdx: 0, limit: 25})
        setPublications(publication_data.arrayof);
        console.log(publications);  // Debugging purposes only
    };

    return (
        <>
            <button onClick={handleClick}>Update List</button>
            <PubCatalog publications={publications} />
        </>
    );
};

export default Home;
