import { FeedAction, FeedActionType, FeedState } from './types';

const initialState: FeedState = {
    postError: null,
    messages: [],
    users: [],
};

export default function feedReducer(
    state: FeedState = initialState,
    action: FeedAction,
): FeedState {
    switch (action.type) {
        case FeedActionType.Load:
            return {
                ...state,
                users: action.payload.users,
                messages: action.payload.messages
                    .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime()),
            };
        case FeedActionType.Posted:
            if (action.payload.error) {
                return {
                    ...state,
                    postError: action.payload.code,
                };
            } else {
                return {
                    ...state,
                    messages: [...state.messages, action.payload.message]
                        .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime()),
                    postError: null,
                };
            }
        case FeedActionType.UserJoined:
            return {
                ...state,
                users: [...state.users, action.payload.user],
            };
        case FeedActionType.UserLeft:
            return {
                ...state,
                users: state.users.filter((user) => user.id !== action.payload.userId),
            };
    }
    return state;
}
