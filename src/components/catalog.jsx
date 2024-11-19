import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import { Avatar, List, ListItemAvatar, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';

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

    return (
        <List>
            {publications.map((publication) => (
                <ListItemButton key={publication.symbol} onClick={() => handleClick(publication)}>
                    <ListItemIcon>
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
