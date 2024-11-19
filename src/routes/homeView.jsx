import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button, Box, Typography } from '@mui/material';
import { Fab } from '@mui/material';
import { Add, OpenInNew } from '@mui/icons-material';
import { open } from '@tauri-apps/plugin-dialog';
import PageLayoutSlider from '../components/transitions';
import { useTranslation, Trans } from 'react-i18next';
import { useNavigate } from 'react-router-dom';

const Home = () => {
    const { t } = useTranslation();
    const navigate = useNavigate();
    
    const handleClick = (_ev) => {
        navigate("/library");
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
        <>
            <PageLayoutSlider>
                <Box
                    sx={{
                        textAlign: "center",
                        paddingTop: 4,
                        paddingBottom: 4,
                        paddingLeft: 16,
                        paddingRight: 16,
                    }}
                >
                    <Typography variant="h4">
                        <Trans i18nKey="home.welcome"/>
                    </Typography>
                    <Typography sx={{paddingBottom: 8}} variant="body1">
                        <Trans i18nKey="home.description"/>
                    </Typography>
                    <Typography variant="body2" color='textSecondary'>
                        <Trans i18nKey="home.disclaimer" components={{plus_icon: <Add/>}}/>
                    </Typography>
                    <div style={{
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        gap: 16,
                        paddingTop: 16,
                        paddingBottom: 16,
                    }}>
                        <Button variant="contained" color='primary' href="https://www.jw.org/" target="_blank">{t("home.download_button")}<OpenInNew sx={{fontSize: "medium", marginLeft: 1}}/></Button>
                        <Button variant="contained" color='primary' onClick={handleClick}>{t("home.local_button")}</Button>
                    </div>
                </Box>
            </PageLayoutSlider>
            <Fab onClick={addPublication} style={{position: "fixed", bottom: 20, right: 20}} color="primary">
                <Add/>
            </Fab>
        </>
    );
};

export default Home;
