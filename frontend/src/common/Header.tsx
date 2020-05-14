import { AppBar, createStyles, Theme, Toolbar, Typography } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import React from 'react';

const useStyles = makeStyles((theme: Theme) => createStyles({
    title: {
        flexGrow: 1,
    },
}));

const Header: React.FC = () => {
    const classes = useStyles();

    return (
        <AppBar position="static">
            <Toolbar>
                <Typography variant="h6" className={classes.title}>
                    Rusty Chat
                </Typography>
            </Toolbar>
        </AppBar>
    );
};

export default Header;
