import Profile from '../components/Profile';

import UserCard_copy from '../components/UserCard_copy';
import { useAuth0 } from "@auth0/auth0-react";
import UsersLayout from '../components/UsersLayout';
import UserCard from '../components/UserCard';

const HomePage = () => {

    const { user, isAuthenticated, isLoading } = useAuth0();


    return (
        <>
            <h2>Home</h2>

            <UsersLayout />

        </>

    );
};

export default HomePage;