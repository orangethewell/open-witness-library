import React, { useState } from 'react';
import { Toolbar, Drawer, AppBar, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Typography, IconButton, Box, Container } from '@mui/material';
import { Outlet, redirect, useNavigate } from 'react-router-dom';
import MenuIcon from '@mui/icons-material/Menu';
import { CollectionsBookmarkTwoTone, HomeTwoTone, SettingsTwoTone } from '@mui/icons-material';
import { useTranslation } from 'react-i18next';
import { useTheme } from '@emotion/react';

const Root = () => {
    const { t } = useTranslation();
    useTheme();

    const [open, setOpen] = useState(false);

    const toggleDrawer = (newOpen) => () => {
        setOpen(newOpen);
    };

    const navigate = useNavigate();

    return (
        <>
            <AppBar position="fixed" sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}>
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
                    <Typography variant="h6" noWrap component="div">
                        Open Witness Library
                    </Typography>
                </Toolbar>
            </AppBar>
            <Drawer 
                sx={{
                    width: 240,
                    flexShrink: 0,
                    [`& .MuiDrawer-paper`]: { width: 240, boxSizing: 'border-box' },
                }}
                open={open} 
                onClose={toggleDrawer(false)}
            >
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
                            <ListItemButton onClick={(_) => navigate("/")}>
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
            <Container className='outlet-container'>
                <Outlet/>
            </Container>
        </>
    );
};

export default Root;