import React, { useEffect, useState } from 'react';
import { Toolbar, Drawer, AppBar, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Typography, IconButton, Box, Container, Slide, useColorScheme, Alert, Collapse, Button, LinearProgress } from '@mui/material';
import { Outlet, redirect, useNavigate } from 'react-router-dom';
import MenuIcon from '@mui/icons-material/Menu';
import { CollectionsBookmarkTwoTone, HomeTwoTone, SettingsTwoTone } from '@mui/icons-material';
import { useTranslation } from 'react-i18next';
import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { TransitionGroup } from 'react-transition-group';
import { platform } from '@tauri-apps/plugin-os';

const Root = () => {
    const { mode, _setMode } = useColorScheme();
    const { t } = useTranslation();
    const [open, setOpen] = useState(false);
    const [alerts, setAlerts] = useState([]);
    const [progress, setProgress] = useState(0);
    const navigate = useNavigate();
    const onMobile = platform() == "android" || platform() == "ios";

    const toggleDrawer = (newOpen) => () => {
        setOpen(newOpen);
    };

    
    useEffect(() => {
        const downloadMissingAssets = async (_ev) => {
            listen("download-progress", (event) => {
                setProgress(event.payload);
                if (event.payload == 100) {
                    setAlerts([...alerts.filter((alert) => {
                        return alert.msgId!== "downloadProgress"
                    })])

                    setAlerts([...alerts, {
                        msgId: "downloadFinished",
                        component: (
                        <Alert 
                            sx={{
                                margin: "10px 0",
                                bgcolor: 'background.paper'
                            }} 
                            variant='outlined' 
                            severity='success'
                        >
                            <span>{t("settings.alerts.download_finished")}</span>
                        </Alert>
                        )
                    }])

                    setTimeout(() => {
                        window.location.reload();
                    }, 5000)
                }
            })

            setAlerts([...alerts.filter((alert) => {
                return alert.msgId!== "downloadFailed" && alert.msgId!== "assetsMissing"
            })])

            setAlerts([...alerts, {
                msgId: "downloadProgress",
                component: (
                <Alert 
                    sx={{
                        margin: "10px 0",
                        bgcolor: 'background.paper'
                    }} 
                    variant='outlined' 
                    severity='info'
                >
                    <span>{t("settings.alerts.download_progress")}</span>
                    <LinearProgress color="inherit" variant="determinate" value={progress} />
                </Alert>
                )
            }])

            await invoke("settings_download_base_assets")
                .catch((_error) => {
                    setAlerts([...alerts, {
                        msgId: "downloadFailed",
                        component: (
                        <Alert 
                            sx={{
                                margin: "10px 0",
                                bgcolor: 'background.paper'
                            }} 
                            variant='outlined' 
                            severity='error'
                            action={
                                <Button onClick={downloadMissingAssets} color="inherit" size="small">{t("settings.alerts.failed_download_button1")}</Button>
                            }
                        >
                            <span>{t("settings.alerts.failed_download")}</span>
                        </Alert>
                    )}])
                })
        }

        invoke("settings_base_assets_present")
            .catch((_error) => {
                setAlerts([...alerts, {
                    msgId: "assetsMissing",
                    component: (
                    <Alert 
                        sx={{
                            margin: "10px 0",
                            bgcolor: 'background.paper'
                        }} 
                        variant='outlined' 
                        severity='warning'
                        action={
                            <Button onClick={downloadMissingAssets} color="inherit" size="small">{t("settings.alerts.missing_assets_button1")}</Button>
                        }
                    >
                        <span>{t("settings.alerts.missing_assets")}</span>
                    </Alert>
                )}])
            });

        let stLink = document.createElement("link")
        stLink.rel = "stylesheet";
        stLink.href = convertFileSrc("collector.css", "appdata");
        document.head.appendChild(stLink);
    }, [])

    useEffect(() => {
        const notifyThemeChange = async () => {
            await invoke("settings_set_webview_theme", {theme: mode});
        }

        notifyThemeChange();
    }, [mode]);

    return (
        <>
            <AppBar position="fixed" sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}>
                {onMobile && <Toolbar sx={{ minHeight: '36px !important', backgroundColor: "#00000033" }}/>}
                <Toolbar sx={{ minHeight: '48px !important' }}>
                    <IconButton
                        color="inherit"
                        aria-label="open drawer"
                        onClick={toggleDrawer(!open)}
                        edge="start"
                        sx={[
                        {
                            mr: 2,
                            boxShadow: "none"
                        },
                        ]}
                    >
                        <MenuIcon />
                    </IconButton>
                    <Typography sx={{ zIndex: (theme) => theme.zIndex.drawer - 1 }} variant="h6" noWrap component="div">
                        Open Witness Library
                    </Typography>
                </Toolbar>
            </AppBar>
            <Container className='outlet-container'>
                {onMobile && <Toolbar sx={{ minHeight: '36px !important' }}/>}
                <List>
                    <TransitionGroup>
                        {alerts.map((alert) => {
                            return <Collapse key={alert.msgId}>{alert.component}</Collapse>
                        })}
                    </TransitionGroup>
                </List>
                <Outlet/>
            </Container>
            <Drawer 
                sx={{
                    width: 240,
                    flexShrink: 0,
                    [`& .MuiDrawer-paper`]: { width: 240, boxSizing: 'border-box' },
                }}
                open={open} 
                onClose={toggleDrawer(false)}
            >
                {onMobile && <Toolbar sx={{ minHeight: '36px !important' }}/>}
                <Toolbar sx={{ minHeight: '48px !important' }}/>
                <Box sx={{ width: 240 }} role="presentation" onClick={toggleDrawer(false)}>
                    <List>
                        <ListItem key={"home"} disablePadding>
                            <ListItemButton onClick={(_) => navigate("/")}>
                            <ListItemIcon>
                                <HomeTwoTone/>
                            </ListItemIcon>
                            <ListItemText primary={t("menu.home")} />
                            </ListItemButton>
                        </ListItem>
                        <ListItem key={"library"} disablePadding>
                            <ListItemButton onClick={(_) => navigate("/library")}>
                            <ListItemIcon>
                                <CollectionsBookmarkTwoTone/>
                            </ListItemIcon>
                            <ListItemText primary={t("menu.library")} />
                            </ListItemButton>
                        </ListItem>
                    </List>
                    <List style={{ position: "absolute", bottom: "0", width: 240 }}>
                        <ListItem key={"settings"} disablePadding>
                            <ListItemButton onClick={(_) => navigate("/settings")}>
                            <ListItemIcon>
                                <SettingsTwoTone/>
                            </ListItemIcon>
                            <ListItemText primary={t("menu.settings")} />
                            </ListItemButton>
                        </ListItem>
                    </List>
                </Box>
            </Drawer>
        </>
    );
};

export default Root;