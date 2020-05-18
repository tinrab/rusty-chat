import { put, StrictEffect, take, takeEvery } from '@redux-saga/core/effects';
import { push } from 'connected-react-router';
import apiActions from '../api/actions';
import apiProto from '../api/proto';
import { ApiActionType, OutputType, ReadApiAction } from '../api/types';
import userActions from './actions';
import { JoinUserAction, UserActionType } from './types';

function* handleJoin(action: JoinUserAction): Generator<StrictEffect> {
    yield put(apiActions.write(apiProto.join(action.payload.name)));

    while (true) {
        const read = (yield take(ApiActionType.Read)) as ReadApiAction;

        if (read.payload.type === OutputType.Error) {
            yield put(userActions.joined({ error: true, code: read.payload.payload.code }));
            break;
        } else if (read.payload.type === OutputType.Joined) {
            const output = read.payload.payload;
            yield put(userActions.joined({
                error: false,
                user: output.user,
                others: output.others,
                messages: output.messages,
            }));
            break;
        }
    }

    yield put(push('/feed'));
}

export function* userSaga(): Generator<StrictEffect> {
    yield takeEvery(UserActionType.Join, handleJoin);
}
