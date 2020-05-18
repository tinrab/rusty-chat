import { OutputError, OutputResult } from '../api/types';
import { UserData } from '../user/types';

export type MessageData = {
    id: string;
    createdAt: Date;
    user: UserData;
    body: string;
};

export type FeedState = {
    messages: MessageData[];
    users: UserData[];
    postError: OutputError | null;
};

export enum FeedActionType {
    Load = 'feed/load',
    Post = 'feed/post',
    Posted = 'feed/posted',
    UserJoined = 'feed/user-joined',
    UserLeft = 'feed/user-left',
}

export type LoadFeedAction = {
    type: FeedActionType.Load;
    payload: {
        messages: MessageData[];
        users: UserData[];
    };
};

export type PostFeedAction = {
    type: FeedActionType.Post;
    payload: {
        body: string;
    };
};

export type PostedFeedAction = {
    type: FeedActionType.Posted;
    payload: OutputResult<PostedFeedActionOk>;
};

export type PostedFeedActionOk = {
    message: MessageData;
};

export type UserJoinedFeedAction = {
    type: FeedActionType.UserJoined;
    payload: { user: UserData };
};

export type UserLeftFeedAction = {
    type: FeedActionType.UserLeft;
    payload: { userId: string; };
};

export type FeedAction =
    LoadFeedAction
    | PostFeedAction
    | PostedFeedAction
    | UserJoinedFeedAction
    | UserLeftFeedAction;
