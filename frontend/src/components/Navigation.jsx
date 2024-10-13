import LoginButton from '../components/LoginButton';
import LogoutButton from '../components/LogoutButton';

const Navigation = () => {
    return (
        <nav>
            <h1>ExperTwice</h1>
            <LoginButton />
            <LogoutButton />
        </nav>
    );
};

export default Navigation;