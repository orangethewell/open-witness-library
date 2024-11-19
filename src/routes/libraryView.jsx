import { Fab, List, ListItemButton, ListItemIcon, ListItemText, useTheme } from '@mui/material';
import { invoke } from '@tauri-apps/api/core';
import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FiBook, FiBookOpen } from "react-icons/fi";
import { GiWhiteTower } from "react-icons/gi";
import { BsExclamation } from "react-icons/bs";
import { TfiAgenda } from "react-icons/tfi";
import { CiBoxList } from "react-icons/ci";
import { GrArticle, GrBook } from "react-icons/gr";
import { GoChecklist } from "react-icons/go";
import { useNavigate } from 'react-router-dom';
import { IoMdPeople } from "react-icons/io";
import { LuRectangleVertical } from "react-icons/lu";
import { PiBooksLight } from "react-icons/pi";
import { open } from '@tauri-apps/plugin-dialog';
import { Add } from '@mui/icons-material';
import PageLayoutSlider from '../components/transitions';


const libraryPublicationTypes = [
    {
        icon: <GrBook />,
        key: "bible",
        types: ["Bible"]
    },
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
        icon: <LuRectangleVertical />,
        key: "tract",
        types: ["Tract"]
    },
    {
        icon: <GrArticle />,
        key: "web",
        types: ["Web"]
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
        icon: <IoMdPeople />,
        key: "kingdom_ministry",
        types: ["Kingdom Ministry"]
    },
    {
        icon: <CiBoxList />,
        key: "program",
        types: ["Program"]
    },
    {
        icon: <PiBooksLight />,
        key: "index",
        types: ["Index"]
    },
    {
        icon: <GoChecklist />,
        key: "manual_guidelines",
        types: ["Manual/Guidelines"]
    }
]

const LibraryView = () => {
    const { t } = useTranslation();

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

    const [availableTypes, setAvailableTypes] = useState([]);
    const navigate = useNavigate();

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

    const theme = useTheme();

    console.log(theme.vars);

    return (
        <>
        <PageLayoutSlider>
        <List>
            {availableTypes.map((publicationType) => (
                    <ListItemButton sx={{
                        paddingTop: 6,
                        paddingBottom: 6,
                        marginBottom: 1,
                        height: 64,
                        backgroundColor: theme.vars.palette.stackButton.main
                    }} key={publicationType.key} onClick={(ev) => {
                        navigate(`/library/${publicationType.key}`);
                    }}>
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
        </PageLayoutSlider>
        <Fab onClick={addPublication} style={{position: "fixed", bottom: 20, right: 20}} color="primary">
            <Add/>
        </Fab>
    </>
    );
};

export default LibraryView;
