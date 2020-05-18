import { OutputResult } from '../api/types';
import { JoinedUserAction, JoinedUserActionOk, JoinUserAction, UserActionType } from './types';

function join(name: string): JoinUserAction {
    return { type: UserActionType.Join, payload: { name } };
}

function joined(result: OutputResult<JoinedUserActionOk>): JoinedUserAction {
    return { type: UserActionType.Joined, payload: result };
}

const userActions = {
    join,
    joined,
};

export default userActions;
