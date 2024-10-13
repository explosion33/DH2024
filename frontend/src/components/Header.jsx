import LoginButton from './LoginButton';
import LogoutButton from './LogoutButton';

const Header = () => {
    return (
        <nav>
            <h1>ExperTwice</h1>
            <LoginButton />
            <LogoutButton />
        </nav>
    );
};

export default Header;