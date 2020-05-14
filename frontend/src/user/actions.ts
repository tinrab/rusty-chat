import { JoinUserAction, UserActionType } from './types';

function join(name: string): JoinUserAction {
    return { type: UserActionType.Join, payload: { name } };
}

const userActions = { join };

export default userActions;
