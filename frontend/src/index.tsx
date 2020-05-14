import { CssBaseline } from '@material-ui/core';
import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import App from './App';
import * as serviceWorker from './serviceWorker';
import configureStore from './store';

const store = configureStore();

ReactDOM.render(
    <React.StrictMode>
        <Provider store={store}>
            <CssBaseline/>
            <App/>
        </Provider>
    </React.StrictMode>,
    document.getElementById('root'),
);

serviceWorker.unregister();
