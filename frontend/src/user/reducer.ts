import { UserAction, UserActionType, UserState } from './types';

const initialState: UserState = {
    currentUser: null,
    joinError: null,
};

export default function userReducer(
    state: UserState = initialState,
    action: UserAction,
): UserState {
    if (action.type === UserActionType.Joined) {
        if (action.payload.error) {
            return {
                ...state,
                joinError: action.payload.code,
                currentUser: null,
            };
        } else {
            return {
                ...state,
                currentUser: {
                    id: action.payload.userId,
                    name: action.payload.name,
                },
            };
        }
    }
    return state;
}
