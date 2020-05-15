import { OutputResult } from '../api/types';
import { JoinedUserAction, JoinedUserOk, JoinUserAction, UserActionType } from './types';

function join(name: string): JoinUserAction {
    return { type: UserActionType.Join, payload: { name } };
}

function joined(result: OutputResult<JoinedUserOk>): JoinedUserAction {
    return { type: UserActionType.Joined, payload: result };
}

const userActions = {
    join,
    joined,
};

export default userActions;
