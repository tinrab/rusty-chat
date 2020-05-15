import { OutputError, OutputResult } from '../api/types';
import { Message } from '../feed/types';

export type User = {
    id: string;
    name: string;
};

export type UserState = {
    currentUser: User | null;
    joinError: OutputError | null;
};

export enum UserActionType {
    Join = 'user/join',
    Joined = 'user/joined',
}

export type JoinUserAction = {
    type: UserActionType.Join;
    payload: { name: string; };
};

export type JoinedUserAction = {
    type: UserActionType.Joined;
    payload: OutputResult<JoinedUserOk>;
};

export type JoinedUserOk = {
    userId: string;
    name: string;
    others: User[];
    messages: Message[];
};

export type UserAction = JoinUserAction | JoinedUserAction;
