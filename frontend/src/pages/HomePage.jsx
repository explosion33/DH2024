import Profile from '../components/Profile';

import UserCard_copy from '../components/UserCard_copy';
import { useAuth0 } from "@auth0/auth0-react";
import UsersLayout from '../components/UsersLayout';
import UserCard from '../components/UserCard';

const HomePage = () => {

    const { user, isAuthenticated, isLoading } = useAuth0();


    return (
        <>
            <UsersLayout
                display="flex"
                justifyContent="center"       // Centers items horizontally
                alignItems="center"           // Centers items vertically (optional)
                flexWrap="wrap"               // Allows items to wrap to the next line if needed
                gap={2}                       // Adds spacing between items
                sx={{ maxWidth: '100%', margin: '2 auto' }} // Centers container itself and limits width
            />

        </>

    );
};

export default HomePage;