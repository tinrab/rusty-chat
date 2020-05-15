import { call, fork, put, StrictEffect, take } from '@redux-saga/core/effects';
import { eventChannel, EventChannel } from 'redux-saga';
import config from '../config';
import apiActions from './actions';
import { ApiActionType, Output, WriteApiAction } from './types';

function createWebSocketChannel(webSocket: WebSocket): EventChannel<Output> {
    return eventChannel<Output>((emit) => {
        webSocket.onmessage = (event): void => {
            const output = JSON.parse(event.data) as Output;
            emit(output);
        };
        return (): void => {
            webSocket.close();
        };
    });
}

function* connectWebSocket(): Generator<StrictEffect> {
    const webSocket = new WebSocket(config.webSocketUrl);
    const webSocketChannel = (yield call(createWebSocketChannel, webSocket)) as EventChannel<Output>;
    yield fork(read, webSocketChannel);
    yield fork(write, webSocket);
}

function* read(webSocketChannel: EventChannel<Output>): Generator<StrictEffect> {
    while (true) {
        const output = (yield take(webSocketChannel)) as Output;
        yield put(apiActions.read(output));
    }
}

function* write(webSocket: WebSocket): Generator<StrictEffect> {
    while (true) {
        const action = (yield take(ApiActionType.Write)) as WriteApiAction;
        webSocket.send(JSON.stringify(action.payload));
    }
}

export default function* apiSaga(): Generator<StrictEffect> {
    yield call(connectWebSocket);
}
