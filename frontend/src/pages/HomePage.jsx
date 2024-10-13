import Profile from '../components/Profile';
import LoginButton from '../components/LoginButton';
import LogoutButton from '../components/LogoutButton';

const HomePage = () => {
    return (
        <>
            <h1>ExperTwice</h1>
            <p>Know a thing or two?<br /> Learn a thing or two.</p>
            <Profile />
            <LoginButton />
            <LogoutButton />
        </>

    );
};

export default HomePage;