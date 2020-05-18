import { Box } from '@material-ui/core';
import React, { HTMLAttributes } from 'react';
import Message from './Message';
import { MessageData } from './types';

type MessageListProps = {
    messages: MessageData[];
} & HTMLAttributes<HTMLDivElement>;

const MessageList: React.FC<MessageListProps> = ({ className, messages }: MessageListProps) => (
    <Box className={className} display="flex" flexDirection="column-reverse" p={2} width="100%" overflow="auto">
        {messages.map((message) => (
            <Message key={message.id} message={message}/>
        ))}
    </Box>
);

export default MessageList;
