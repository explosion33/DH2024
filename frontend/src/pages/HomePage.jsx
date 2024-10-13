import Profile from '../hooks/Profile';
import LoginButton from '../components/LoginButton';
import LogoutButton from '../components/LogoutButton';

const HomePage = () => {
    return (
        <>
            <h1>Home</h1>
            <Profile />
            <LoginButton />
            <LogoutButton />
        </>

    );
};

export default HomePage;