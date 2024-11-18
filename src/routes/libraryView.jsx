import { List, ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';
import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FiBook, FiBookOpen } from "react-icons/fi";
import { GiWhiteTower } from "react-icons/gi";
import { BsExclamation } from "react-icons/bs";
import { TfiAgenda } from "react-icons/tfi";
import { CiBoxList } from "react-icons/ci";
import { GrArticle } from "react-icons/gr";
import { GoChecklist } from "react-icons/go";

const libraryPublicationTypes = [
    {
        icon: <FiBook />,
        key: "book",
        types: ["Book"],
    },
    {
        icon: <FiBookOpen />,
        key: "brochure",
        types: ["Brochure", "Booklet"]
    },
    {
        icon: <GiWhiteTower />,
        key: "watchtower",
        types: ["Watchtower"]
    },
    {
        icon: <BsExclamation />,
        key: "awake",
        types: ["Awake!"]
    },
    {
        icon: <TfiAgenda />,
        key: "meeting_workbook",
        types: ["Meeting Workbook"]
    },
    {
        icon: <CiBoxList />,
        key: "program",
        types: ["Program"]
    },
    {
        icon: <GrArticle />,
        key: "web",
        types: ["Web"]
    },
    {
        icon: <GoChecklist />,
        key: "manual_guidelines",
        types: ["Manual/Guidelines"]
    }
]

const LibraryView = () => {
    const { t } = useTranslation();

    const [availableTypes, setAvailableTypes] = useState([]);
    const appendAvailableType = (pubType) => {
        setAvailableTypes((prevTypes) => [...prevTypes, pubType]);
    };

    const fetchCounts = async () => {
        const updatedTypes = await Promise.all(
            libraryPublicationTypes.map(async (publicationType) => {
                let count = 0;

                for (const pubType of publicationType.types) {
                    const type_count = await invoke("catalog_get_count_from_type", { publicationType: pubType });
                    count += type_count;
                }

                if (count > 0) {
                    return {
                        key: publicationType.key,
                        types: publicationType.types,
                        icon: publicationType.icon
                    };
                }

                return null;
            })
        );

        setAvailableTypes(updatedTypes.filter((type) => type !== null));
    };

    useEffect(() => {
        fetchCounts();
    }, []);

    return (
        <List>
            {availableTypes.map((publicationType) => (
                    <ListItemButton sx={{
                        paddingTop: 6,
                        paddingBottom: 6,
                        marginBottom: 1,
                        height: 64,
                        backgroundColor: "#FFFFFF08"
                    }} key={publicationType.key} onClick={(ev) => {}}>
                        <ListItemIcon sx={{
                            fontSize: 36,
                            marginRight: 2,
                        }}>
                            {publicationType.icon}
                        </ListItemIcon>
                        <ListItemText
                            primaryTypographyProps={{
                                variant: "h5"
                            }}
                            primary={t(`library.publication_types.${publicationType.key}`)}
                        />
                    </ListItemButton>
                )
            )}
        </List>
    );
};

export default LibraryView;
