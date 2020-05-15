import { Box, Button, TextField, Typography } from '@material-ui/core';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import React, { ChangeEvent, useState } from 'react';
import { useDispatch } from 'react-redux';
import userActions from '../user/actions';

const useStyles = makeStyles((theme: Theme) => createStyles({
    joinButton: {
        marginLeft: theme.spacing(1),
    },
}));

type HomeState = {
    name: string;
    nameValid: boolean;
};

const Home: React.FC = () => {
    const classes = useStyles();
    const [state, setState] = useState<HomeState>({ name: '', nameValid: false });
    const dispatch = useDispatch();

    const isNameValid = (name: string) => name.length >= 4 && name.length <= 24;

    const handleNameChange = (event: ChangeEvent<HTMLInputElement>) => {
        const name = event.target.value;
        setState((prevState) => ({
            ...prevState,
            name,
            nameValid: isNameValid(name.trim()),
        }));
    };

    const handleJoin = () => {
        const name = state.name.trim();
        if (!isNameValid(name)) {
            return;
        }
        dispatch(userActions.join(name));
    };

    return (
        <Box display="flex" flexDirection="column" textAlign="center" flexGrow={1} mt={4}>
            <Typography variant="h3">
                Welcome!
            </Typography>
            <Box display="flex" justifyContent="center" mt={2}>
                <TextField label="Name" value={state.name} onChange={handleNameChange}/>
                <Button className={classes.joinButton} variant="contained" color="primary"
                        disabled={!state.nameValid} onClick={handleJoin}>
                    Join
                </Button>
            </Box>

        </Box>
    );
};

export default Home;
