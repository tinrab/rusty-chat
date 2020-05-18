import { Box, Button, TextField, Typography } from '@material-ui/core';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import React, { ChangeEvent, FormEvent, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { OutputError } from '../api/types';
import { AppState } from '../store';
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

    const joinedErrorCode = useSelector((state: AppState) => state.user.joinError);
    let joinedError = null;
    if (state.name.trim().length !== 0) {
        switch (joinedErrorCode) {
            case OutputError.InvalidName:
                joinedError = 'Invalid name.';
                break;
            case OutputError.NameTaken:
                joinedError = 'Name is taken.';
                break;
        }
    }

    const isNameValid = (name: string) => name.length >= 4 && name.length <= 24;

    const handleNameChange = (event: ChangeEvent<HTMLInputElement>) => {
        const name = event.target.value;
        setState((prevState) => ({
            ...prevState,
            name,
            nameValid: isNameValid(name.trim()),
        }));
    };

    const handleJoin = (e: FormEvent) => {
        e.preventDefault();
        const name = state.name.trim();
        if (!isNameValid(name)) {
            return;
        }
        dispatch(userActions.join(name));
    };

    return (
        <Box display="flex" flexDirection="column" textAlign="center" flexGrow={1} pt={4}>
            <Typography variant="h3">
                Welcome!
            </Typography>
            <Box component="form" onSubmit={handleJoin} display="flex" justifyContent="center" alignItems="baseline"
                 mt={2}>
                <TextField label="Name" value={state.name} onChange={handleNameChange} error={!!joinedError}
                           helperText={joinedError}/>
                <Button className={classes.joinButton} variant="contained" color="primary"
                        disabled={!state.nameValid} onClick={handleJoin}>
                    Join
                </Button>
            </Box>
        </Box>
    );
};

export default Home;
