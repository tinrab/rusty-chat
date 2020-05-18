import { makeStyles } from '@material-ui/core/styles';
import React from 'react';
import { Route, Switch } from 'react-router-dom';
import Header from './common/Header';
import Feed from './feed/Feed';
import Home from './home/Home';

const useStyles = makeStyles({
    '@global': {
        html: {
            height: '100%',
        },
        body: {
            height: '100%',
        },
        '#root': {
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
        },
    },
});

const App: React.FC = () => {
    useStyles();
    return (
        <>
            <Header/>
            <Switch>
                <Route exact path="/" component={Home}/>
                <Route path="/feed" component={Feed}/>
            </Switch>
        </>
    );
};

export default App;
