import { OutputError, OutputResult } from '../api/types';
import { MessageData } from '../feed/types';

export type UserData = {
    id: string;
    name: string;
};

export type UserState = {
    currentUser: UserData | null;
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
    payload: OutputResult<JoinedUserActionOk>;
};

export type JoinedUserActionOk = {
    user: UserData;
    others: UserData[];
    messages: MessageData[];
};

export type UserAction = JoinUserAction | JoinedUserAction;
