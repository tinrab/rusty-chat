import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Header from './common/Header';
import Feed from './feed/Feed';
import Home from './home/Home';

const App: React.FC = () => {
    return (
        <Router>
            <Header/>
            <Switch>
                <Route exact path="/">
                    <Home/>
                </Route>
                <Route path="/feed">
                    <Feed/>
                </Route>
            </Switch>
        </Router>
    );
};

export default App;
