import { Box, Typography } from '@material-ui/core';
import React, { HTMLAttributes } from 'react';
import { UserData } from '../user/types';
import UserAvatar from '../user/UserAvatar';

type UserProps = {
    user: UserData;
} & HTMLAttributes<HTMLDivElement>;

const User: React.FC<UserProps> = ({ className, user }: UserProps) => (
    <Box className={className} display="flex" p={1}>
        <Box mr={1}>
            <UserAvatar user={user}/>
        </Box>
        <Box alignSelf="center">
            <Typography variant="body1">{user.name}</Typography>
        </Box>
    </Box>
);

export default User;
