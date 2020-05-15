import { call, fork, put, StrictEffect, take, takeEvery } from '@redux-saga/core/effects';
import { eventChannel, EventChannel } from '@redux-saga/core/types';
import config from '../config';
import { JoinUserAction, UserActionType } from '../user/types';
import { Input, Output } from './types';

function createWebSocketChannel(webSocket: WebSocket): EventChannel<Output> {
    return eventChannel<Input>((emit) => {
        webSocket.onopen = (): void => {

        };
        webSocket.onmessage = (): void => {

        };
        webSocket.onclose = (): void => {

        };
        return (): void => {
            webSocket.close();
        };
    });
}

function* connectWebSocket(): Generator<StrictEffect> {
    const webSocket = new WebSocket(config.webSocketUrl);
    const webSocketChannel = (yield call(createWebSocketChannel, webSocket)) as EventChannel<Output>;
    yield fork(read, webSocket, webSocketChannel);
    yield fork(write, webSocket);
}

function* read(webSocket: WebSocket, webSocketChannel: EventChannel<Output>): Generator<StrictEffect> {
    while (true) {
        const output = (yield take(webSocketChannel)) as Output;
        yield put(output);
    }
}

function *write(webSocket: WebSocket): Generator<StrictEffect> {
    while (true) {
        if (webSocket.readyState !== WebSocket.OPEN) {
            break;
        }
        const 
        webSocket.send(JSON.stringify());
    }
}

function* handleUserJoin(action: JoinUserAction) {
    console.log(action);
}

export default function* apiSaga(): Generator<StrictEffect> {
    yield takeEvery(UserActionType.Join, handleUserJoin);
}
