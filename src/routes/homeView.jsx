import React, { useState } from 'react';
import Catalog from '../components/catalog';
import { invoke } from '@tauri-apps/api/core';
import { Button, Box } from '@mui/material';
import { Fab } from '@mui/material';
import { Add } from '@mui/icons-material';
import { open } from '@tauri-apps/plugin-dialog';

const Home = () => {
    const [publications, setPublications] = useState([]);
    
    const handleClick = async () => {
        let publication_data = await invoke("catalog_get_list_from_type", {publicationType: "Book"})
        setPublications(publication_data);
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
        
        await invoke("catalog_install_jwpub_file", {filePath: file})
    }

    return (
        <Box>
            <Button onClick={handleClick}>Update List</Button>
            <Catalog publications={publications} />
            <Fab onClick={addPublication} style={{position: "fixed", bottom: 20, right: 20}} color="primary">
                <Add/>
            </Fab>
        </Box>
    );
};

export default Home;
