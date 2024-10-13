import LoginButton from './LoginButton';
import LogoutButton from './LogoutButton';
import { useAuth0 } from '@auth0/auth0-react';

const Header = () => {

    const { user, isAuthenticated, isLoading } = useAuth0();

    return (
        <nav>
            <h1>ExperTwice</h1>
            {!isAuthenticated ? <LoginButton />
                : <LogoutButton />
            }
        </nav>
    );
};

export default Header;