import createSagaMiddleware from 'redux-saga';
import { applyMiddleware, combineReducers, createStore } from 'redux';
import apiSaga from './api/saga';
import userReducer from './user/reducer';
import { UserAction } from './user/types';

const rootReducer = combineReducers({
    user: userReducer,
});

export type AppState = ReturnType<typeof rootReducer>;
export type AppAction = UserAction;

export default function configureStore(): any {
    const sagaMiddleware = createSagaMiddleware();
    const store = createStore(rootReducer, applyMiddleware(sagaMiddleware));

    sagaMiddleware.run(apiSaga);

    return store;
}
