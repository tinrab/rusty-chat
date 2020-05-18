import { Box, createStyles, List, Theme, Typography } from '@material-ui/core';
import { makeStyles } from '@material-ui/core/styles';
import React, { HTMLAttributes } from 'react';
import { UserData } from '../user/types';
import User from './User';

const useStyles = makeStyles((theme: Theme) => createStyles({
    list: {
        padding: 0,
    },
}));

type UserListProps = {
    users: UserData[];
} & HTMLAttributes<HTMLDivElement>;

const UserList: React.FC<UserListProps> = ({ className, users }: UserListProps) => {
    const classes = useStyles();

    return (
        <Box className={className} p={2}>
            <Typography variant="h5" gutterBottom>Users ({users.length})</Typography>
            <List className={classes.list}>
                {users.map((user) => <User key={user.id} user={user}/>)}
            </List>
        </Box>
    );
};

export default UserList;
