import { applyMiddleware, combineReducers, createStore } from 'redux';
import createSagaMiddleware from 'redux-saga';
import apiSaga from './api/saga';
import userReducer from './user/reducer';
import { userSaga } from './user/saga';

const rootReducer = combineReducers({
    user: userReducer,
});

export type AppState = ReturnType<typeof rootReducer>;

export default function configureStore(): any {
    const sagaMiddleware = createSagaMiddleware();
    const store = createStore(rootReducer, applyMiddleware(sagaMiddleware));

    sagaMiddleware.run(apiSaga);
    sagaMiddleware.run(userSaga);

    return store;
}
