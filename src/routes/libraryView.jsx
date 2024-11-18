import { List, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';
import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

const libraryPublicationTypes = [
    {
        key: "book",
        types: ["Book"],
    },
    {
        key: "brochure",
        types: ["Brochure", "Booklet"]
    },
    {
        key: "watchtower",
        types: ["Watchtower"]
    },
    {
        key: "awake",
        types: ["Awake!"]
    },
    {
        key: "meeting_workbook",
        types: ["Meeting Workbook"]
    },
    {
        key: "program",
        types: ["Program"]
    },
    {
        key: "web",
        types: ["Web"]
    },
    {
        key: "manual_guidelines",
        types: ["Manual/Guidelines"]
    }
]

const LibraryView = () => {
    const { t } = useTranslation();

    const [availableTypes, setAvailableTypes] = useState([]);
    const appendAvailableType = (pubType) => {
        setAvailableTypes(availableTypes.push(pubType))
    }

    useEffect(() => {
        libraryPublicationTypes.map(async (publicationType) => {
            let count = 0;

            publicationType.types.map(async (pubType) => {
                let type_count = await invoke("catalog_get_count_from_type", {publicationType: pubType});
                count += type_count;
            })

            if (count > 0) {
                appendAvailableType({
                    key: publicationType.key,
                    types: publicationType.types
                })
            }
        })
    })

    return (
        <List>
            {availableTypes.map((publicationType) => {
                    <ListItemButton key={publicationType.key} onClick={(ev) => {}}>
                        <ListItemIcon>
                        </ListItemIcon>
                        <ListItemText
                            primary={t(`library.publication_types.${publicationType.key}`)}
                        />
                    </ListItemButton>
                }
            )}
        </List>
    );
};

export default LibraryView;
