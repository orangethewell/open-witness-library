import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { Avatar, List, ListItemAvatar, ListItemButton, ListItemIcon, ListItemText, useTheme } from '@mui/material';

const PublicationIcon = ({type, id, alt=""}) => {
    let [icon, SetIcon] = useState("");

    useEffect(() => {
        const getImages = async(type, id) => {
            let images = await invoke(`catalog_get_images_of_type`, {imageType: type, publicationId: id});
            SetIcon(convertFileSrc(images[0].path))
        }

        getImages(type, id)
    }, [])

    return (
        <img
            style={{marginRight: 20, height: 90}} 
            src={icon}
            alt={alt}
        />
    )
}

const Catalog = ({ publications }) => {
    const navigate = useNavigate();

    const handleClick = (publication) => {
        navigate(`/publication/${publication.jwpub.replace(".jwpub", "")}`);
    };

    const theme = useTheme();

    return (
        <List>
            {publications.map((publication) => (
                <ListItemButton sx={{
                    paddingTop: 5.68,
                    paddingBottom: 5.68,
                    marginBottom: 1,
                    height: 64,
                    backgroundColor: theme.vars.palette.stackButton.main
                }} key={publication.symbol} onClick={() => handleClick(publication)}>
                    <ListItemIcon sx={{
                        marginLeft: -1.99
                    }}>
                        <PublicationIcon type="t" id={publication.id} alt={publication.title} />
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

export default Catalog;
