import React, { useState } from 'react';
import PubCatalog from '../components/catalog';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@mui/material';
import { Fab } from '@mui/material';
import { Add } from '@mui/icons-material';
import { open } from '@tauri-apps/plugin-dialog';

const Home = () => {
    const [publications, setPublications] = useState([]);
    
    const handleClick = async () => {
        let publication_data = await invoke("pubcatalog_get_list_from", {lang: 'T', category: 'bk', startIdx: 0, limit: 25})
        setPublications(publication_data.arrayof);
        console.log(publications);  // Debugging purposes only
    };

    const addPublication = async () => {
        const file = await open({
            multiple: false,
            filters: [{
                name: "JWPUB file",
                extensions: ["jwpub"],
            }],
            directory: false,
        })
        console.log("Trying install file");
        await invoke("pubcatalog_install_jwpub_file", {pubPath: file})
        console.log("Something wrong?")
    }

    return (
        <>
            <Button onClick={handleClick}>Update List</Button>
            <PubCatalog publications={publications} />
            <Fab onClick={addPublication} style={{position: "fixed", bottom: 20, right: 20}} color="primary">
                <Add/>
            </Fab>
        </>
    );
};

export default Home;
