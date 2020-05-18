import { Box, Typography } from '@material-ui/core';
import React, { HTMLAttributes } from 'react';
import * as timeago from 'timeago.js';
import UserAvatar from '../user/UserAvatar';
import { MessageData } from './types';

type MessageProps = {
    message: MessageData;
} & HTMLAttributes<HTMLDivElement>;

const Message: React.FC<MessageProps> = ({ className, message }: MessageProps) => (
    <Box className={className} display="flex" p={1}>
        <Box mr={1}>
            <UserAvatar user={message.user}/>
        </Box>
        <Box>
            <Typography variant="body1">{message.body}</Typography>
            <Typography
                component="span"
                variant="body2"
                color="textSecondary"
            >
                {`${message.user.name} - ${timeago.format(message.createdAt)}`}
            </Typography>
        </Box>
    </Box>
);

export default Message;
