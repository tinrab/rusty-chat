export enum ApiActionType {
    Write = 'api/write',
    Read = 'api/read',
}

export type WriteApiAction = {
    type: ApiActionType.Write;
    payload: Input;
};

export type ReadApiAction = {
    type: ApiActionType.Read;
    payload: Output;
};

export type ApiAction = WriteApiAction | ReadApiAction;

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
    Posted = 'posted',
    UserPosted = 'user-posted',
}

export enum OutputError {
    NameTaken = 'name-taken',
    InvalidName = 'invalid-name',
    NotJoined = 'not-joined',
    InvalidMessageBody = 'invalid-message-body',
}

export type OutputResult<T> = T & { error: false } | {
    error: true;
    code: OutputError;
};

export type UserOutput = {
    id: string;
    name: string;
};

export type MessageOutput = {
    id: string;
    user: UserOutput;
    body: string;
    createdAt: Date,
};

export type ErrorOutput = {
    type: OutputType.Error;
    payload: { code: OutputError };
};

export type AliveOutput = {
    type: OutputType.Alive;
};

export type JoinedOutput = {
    type: OutputType.Joined;
    payload: {
        user: UserOutput;
        others: UserOutput[];
        messages: MessageOutput[];
    };
};

export type UserJoinedOutput = {
    type: OutputType.UserJoined;
    payload: {
        user: UserOutput;
    };
};

export type UserLeftOutput = {
    type: OutputType.UserLeft;
    payload: {
        userId: string;
    };
};

export type PostedOutput = {
    type: OutputType.Posted;
    payload: {
        message: MessageOutput;
    };
};

export type UserPostedOutput = {
    type: OutputType.UserPosted;
    payload: {
        message: MessageOutput;
    };
};

export type Output =
    ErrorOutput |
    AliveOutput |
    JoinedOutput |
    UserJoinedOutput |
    UserLeftOutput |
    PostedOutput |
    UserPostedOutput;
