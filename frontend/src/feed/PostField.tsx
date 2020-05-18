import { Box, Button, createStyles, TextField, Theme, Typography } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import React, { ChangeEvent, FormEvent, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { OutputError } from '../api/types';
import { AppState } from '../store';
import { UserData } from '../user/types';
import feedActions from './actions';

const useStyles = makeStyles((theme: Theme) => createStyles({
    messageInput: {
        marginLeft: theme.spacing(2),
        flexGrow: 1,
    },
    postButton: {
        marginLeft: theme.spacing(1),
    },
}));

type PostFieldProps = {
    user: UserData;
};

type PostFieldState = {
    body: string;
    bodyValid: boolean;
};

const PostField: React.FC<PostFieldProps> = ({ user }: PostFieldProps) => {
    const classes = useStyles();
    const [state, setState] = useState<PostFieldState>({ body: '', bodyValid: false });
    const dispatch = useDispatch();

    const postErrorCode = useSelector((state: AppState) => state.feed.postError);
    let postError = null;
    if (state.body.trim().length !== 0) {
        switch (postErrorCode) {
            case OutputError.NotJoined:
                postError = 'Not joined.';
                break;
            case OutputError.InvalidMessageBody:
                postError = 'Invalid message.';
                break;
        }
    }

    const isBodyValid = (body: string) => body.length > 0 && body.length <= 256;

    const handleBodyChange = (event: ChangeEvent<HTMLInputElement>) => {
        const body = event.target.value;
        setState((prevState) => ({
            ...prevState,
            body,
            bodyValid: isBodyValid(body.trim()),
        }));
    };

    const handlePost = (e: FormEvent) => {
        e.preventDefault();
        const body = state.body.trim();
        if (!isBodyValid(body)) {
            return;
        }
        dispatch(feedActions.post(body));
        setState((prevState) => ({ body: '', bodyValid: false }));
    };

    return (
        <Box component="form" onSubmit={handlePost} display="flex" justifyContent="center" alignItems="baseline">
            <Typography variant="body1">{user.name}</Typography>
            <TextField className={classes.messageInput} label="Say..." value={state.body}
                       onChange={handleBodyChange}
                       error={!!postError}
                       helperText={postError}/>
            <Button className={classes.postButton} variant="contained" color="primary"
                    disabled={!state.bodyValid} onClick={handlePost}>
                Send
            </Button>
        </Box>
    );
};

export default PostField;
