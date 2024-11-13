import React, { useState } from 'react';
import { Button, Divider, Toolbar, Drawer, AppBar, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Typography, IconButton, Box } from '@mui/material';
import { Outlet, redirect, useNavigate } from 'react-router-dom';
import MenuIcon from '@mui/icons-material/Menu';
import { HomeTwoTone } from '@mui/icons-material';

const Root = () => {
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
                <Box sx={{ width: 250 }} role="presentation" onClick={toggleDrawer(false)}>
                    <List>
                        <ListItem key={"Home"} disablePadding>
                            <ListItemButton onClick={(_) => navigate("/")}>
                            <ListItemIcon>
                                <HomeTwoTone/>
                            </ListItemIcon>
                            <ListItemText primary={"Home"} />
                            </ListItemButton>
                        </ListItem>
                    </List>
                </Box>
            </Drawer>
            <div className='outlet-container'>
                <Outlet/>
            </div>
        </>
    );
};

export default Root;