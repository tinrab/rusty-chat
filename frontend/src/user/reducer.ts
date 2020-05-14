import { UserAction, UserActionType, UserState } from './types';

const initialState: UserState = {};

export default function userReducer(
    state: UserState = initialState,
    action: UserAction,
): UserState {
    if (action.type === UserActionType.Joined) {
        return {
            ...state,
            currentUser: action.payload,
        };
    }
    return state;
}
