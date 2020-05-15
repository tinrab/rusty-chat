export enum InputType {
    Join = 'join',
    Post = 'post',
}

export type JoinInput = {
    type: InputType.Join;
    payload: { name: string; };
};

export type PostInput = {
    type: InputType.Post;
    payload: { body: string; };
};

export type Input = JoinInput | PostInput;

export enum OutputType {
    Error = 'error',
    Alive = 'alive',
    Joined = 'joined',
    UserJoined = 'user-joined',
    UserLeft = 'user-left',
    UserPosted = 'user-posted',
}

export enum OutputError {
    NameTaken = 'NameTaken',
    InvalidName = 'InvalidName',
    NotJoined = 'NotJoined',
    InvalidMessageBody = 'InvalidMessageBody',
}

export type UserOutput = {
    id: string;
    name: string;
};

export type MessageOutput = {
    id: string;
    user: UserOutput;
    body: string;
    created_at: Date,
};

export type ErrorOutput = {
    type: OutputType.Error;
    payload: OutputError;
};

export type AliveOutput = {
    type: OutputType.Alive;
};

export type JoinedOutput = {
    type: OutputType.Joined;
    userId: string;
    others: UserOutput[];
    messages: MessageOutput[];
};

export type UserJoinedOutput = {
    type: OutputType.UserJoined;
    user: UserOutput;
};

export type UserLeftOutput = {
    type: OutputType.UserLeft;
    userId: string;
};

export type UserPostedOutput = {
    type: OutputType.UserPosted;
    message: MessageOutput;
};

export type Output =
    ErrorOutput |
    AliveOutput |
    JoinedOutput |
    UserJoinedOutput |
    UserLeftOutput |
    UserPostedOutput;
