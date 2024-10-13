import { useNavigate } from 'react-router-dom';
import { useAuth0 } from "@auth0/auth0-react";
import { useState } from 'react';

const HomePage = () => {

    const navigate = useNavigate();
    const { isAuthenticated, user } = useAuth0();
    const { dest, setDest } = useState('');
    
    if(isAuthenticated && user){
        fetch('https://e2.armstronglabs.net/api/info/'+user?.sub, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        }).then((response) => {
            if (response.status === 200) {
                setDest('/people');
            } else {
                setDest('/onboarding');
            }
        });
    }

    return (
        <>
            <header className="question">
                <h1>ExperTwice</h1>
            </header>
            <main>
                <center>
                    <img src="./logo.svg" alt="ExperTwice" />
                    <div>
                        <p>Know a thing or two?</p>
                        <p>Learn a thing or two.</p>
                        <button onClick={() => navigate(dest)}>{dest == '/onboarding' ? 'Sign Up | Log In' : 'Open App'}</button>
                    </div>
                </center>
            </main>
        </>

    );
};

export default HomePage;