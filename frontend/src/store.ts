import { connectRouter, routerMiddleware } from 'connected-react-router';
import { createBrowserHistory } from 'history';
import { applyMiddleware, combineReducers, compose, createStore } from 'redux';
import createSagaMiddleware from 'redux-saga';
import apiSaga from './api/saga';
import feedReducer from './feed/reducer';
import { feedSaga } from './feed/saga';
import userReducer from './user/reducer';
import { userSaga } from './user/saga';

export const history = createBrowserHistory();

const rootReducer = combineReducers({
    router: connectRouter(history),
    user: userReducer,
    feed: feedReducer,
});

export type AppState = ReturnType<typeof rootReducer>;

export default function configureStore(): any {
    const sagaMiddleware = createSagaMiddleware();
    const store = createStore(rootReducer, compose(applyMiddleware(routerMiddleware(history), sagaMiddleware)));

    sagaMiddleware.run(apiSaga);
    sagaMiddleware.run(userSaga);
    sagaMiddleware.run(feedSaga);

    return store;
}
