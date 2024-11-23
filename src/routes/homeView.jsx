import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button, Box, Typography, useTheme } from '@mui/material';
import { Fab } from '@mui/material';
import { Add, OpenInNew } from '@mui/icons-material';
import PageLayoutSlider from '../components/transitions';
import { useTranslation, Trans } from 'react-i18next';
import { useNavigate } from 'react-router-dom';
import { addPublication } from "../common";

const Home = () => {
    const theme = useTheme();
    const { t } = useTranslation();
    const navigate = useNavigate();
    
    const handleClick = (_ev) => {
        navigate("/library");
    };

    return (
        <>
            <PageLayoutSlider>
                <Box
                    sx={{
                        textAlign: "center",
                        paddingTop: 4,
                        paddingBottom: 4,
                        [theme.breakpoints.up('md')]: {
                            paddingLeft: 16,
                            paddingRight: 16,
                        },
                        paddingLeft: 4,
                        paddingRight: 4,
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
