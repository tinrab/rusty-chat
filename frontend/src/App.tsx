import React from 'react';
import { useSelector } from 'react-redux';
import { BrowserRouter as Router, Route, Switch,
    useHistory,
    useLocation } from 'react-router-dom';
import Header from './common/Header';
import Feed from './feed/Feed';
import Home from './home/Home';
import { AppState } from './store';

const App: React.FC = () => {
    const history = useHistory();
    const location = useLocation();
    const user = useSelector((state: AppState) => state.user.currentUser);

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
