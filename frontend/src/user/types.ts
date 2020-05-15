export type User = {
    id: string;
    name: string;
};

export type UserState = {
    currentUser?: User;
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
    payload: User;
};

export type UserAction = JoinUserAction | JoinedUserAction;
