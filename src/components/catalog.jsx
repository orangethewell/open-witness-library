import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { Avatar, List, ListItemAvatar, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';


const PubCatalog = ({ publications }) => {
    const [loadedIdx, setLoadedIdx] = useState(0);  // `loaded_idx`
    const limit = 10;  // Defina o limite conforme necessário
    const navigate = useNavigate();

    console.log(publications)

    const handleClick = async (publication) => {
        let pubSymbol = `${publication.symbol}_${publication.language}`;  // TODO: Remover este sufixo
        await invoke("pubcatalog_set_media_location", {lang: 'T', category: 'bk', pubSymbol})
        navigate(`/summary/${publication.language}/${publication.category}/${pubSymbol}`);
    };

    return (
        <List>
            {publications.map((publication) => (
                <ListItemButton key={publication.symbol} onClick={() => handleClick(publication)}>
                    <ListItemIcon>
                        <img style={{marginRight: 20, height: 90}} src={convertFileSrc(publication.cover_icon_path)} alt={publication.title} />
                    </ListItemIcon>
                    <ListItemText
                        primary={publication.title}
                        secondary={publication.year}
                    />
                </ListItemButton>
            ))}
        </List>
    );
};

export default PubCatalog;
