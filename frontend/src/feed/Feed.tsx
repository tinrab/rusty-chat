import React from 'react';
import { useSelector } from 'react-redux';
import { Redirect, useLocation } from 'react-router-dom';
import { AppState } from '../store';

const Feed: React.FC = () => {
    const location = useLocation();
    const user = useSelector((state: AppState) => state.user.currentUser);

    return user!! ? (
        <div>Feed</div>
    ) : <Redirect to={{ pathname: '/', state: { from: location } }}/>;
};

export default Feed;
