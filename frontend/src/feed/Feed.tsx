import { Box, createStyles, Theme } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import React from 'react';
import { useSelector } from 'react-redux';
import { Redirect, useLocation } from 'react-router-dom';
import { AppState } from '../store';
import MessageList from './MessageList';
import PostField from './PostField';
import UserList from './UserList';

const useStyles = makeStyles((theme: Theme) => createStyles({
    content: {
        borderBottomColor: theme.palette.divider,
        borderBottomStyle: 'solid',
        borderBottomWidth: 1,
    },
    userList: {
        borderRightColor: theme.palette.divider,
        borderRightStyle: 'solid',
        borderRightWidth: 1,
    },
}));

const Feed: React.FC = () => {
    const classes = useStyles();
    const location = useLocation();
    const user = useSelector((state: AppState) => state.user.currentUser);
    const { messages, users } = useSelector((state: AppState) => state.feed);

    return !!user ? (
        <Box display="flex" flexDirection="column" flexGrow={1} minHeight={0}>
            <Box className={classes.content} display="flex" flexGrow={1} minHeight={0} width="100%">
                <Box className={classes.userList} width={200}>
                    <UserList users={users}/>
                </Box>
                <MessageList messages={messages}/>
            </Box>
            <Box p={2}>
                <PostField user={user}/>
            </Box>
        </Box>
    ) : <Redirect to={{ pathname: '/', state: { from: location } }}/>;
};

export default Feed;
