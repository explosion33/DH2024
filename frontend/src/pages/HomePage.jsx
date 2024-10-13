import { useNavigate } from 'react-router-dom';
import { useAuth0 } from "@auth0/auth0-react";

const HomePage = () => {

    const navigate = useNavigate();
    const { isAuthenticated, user } = useAuth0();
    
    if(isAuthenticated && user){
        try{
            fetch('http://e2.armstronglabs.net/info/'+user?.sub, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
            }).then((response) => {
                if (response.status === 200) {
                    navigate('/people');
                } else {
                    navigate('/onboarding');
                }
            });
        } catch {
            navigate('/people');
        }
    }

    return (
        <>
            <div>
                <h1>ExperTwice</h1>
            </div>
            <main>
                <img />
                <div>
                    <p>Know a thing or two?</p>
                    <p>Learn a thing or two.</p>
                    <button onClick={() => navigate('/onboarding')}>Sign Up | Log In</button>
                </div>
            </main>
        </>

    );
};

export default HomePage;