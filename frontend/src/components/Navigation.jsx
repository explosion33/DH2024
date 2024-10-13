import LoginButton from '../components/LoginButton';
import LogoutButton from '../components/LogoutButton';
import { useAuth0 } from '@auth0/auth0-react';


const Navigation = () => {

    const { user, isAuthenticated, isLoading } = useAuth0();

    return (
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <h1>ExperTwice</h1>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                {isAuthenticated ? <p>Hi, {user.name}</p> : <p></p>}
                {!isAuthenticated ? <LoginButton /> : <LogoutButton />}
            </div>
        </div>
    );
};

export default Navigation;