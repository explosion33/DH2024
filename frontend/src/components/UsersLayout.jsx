import { useState, useEffect } from "react";
import Stack from '@mui/material/Stack';
import UserCard from './UserCard';
import { useAuth0 } from "@auth0/auth0-react";

const UsersLayout = () => {
    const { user } = useAuth0();
    const [users, setUsers] = useState([]);  // useState to track users

    useEffect(() => {
        const getUsers = async () => {
            if (!user) return;  // Ensure that 'user' is defined

            console.log("Fetching users");
            try {
                const response = await fetch('https://e2.armstronglabs.net/api/matches/' + user.sub + '/100', {
                    method: "GET",
                    headers: {
                        "Content-Type": "application/json",
                        "Accept": "application/json"
                    },
                });

                const data = await response.json();
                const fetchedUsers = data.matches.map(match => match.uid);  // Extract the users
                setUsers(fetchedUsers);  // Update state
            } catch (error) {
                console.error("Error fetching users:", error);
            }
        };

        getUsers();
    }, [user]);  // Dependency array - run this effect when 'user' changes

    return (
        <Stack
            spacing={3}
            direction="row"
            useFlexGap
            justifyContent="center"       // Centers items horizontally
            alignItems="center"  
            sx={{ flexWrap: 'wrap', margin: '5% auto' }}
        >
            {users.length === 0 ? (
                <p>Loading users...</p>  // Show a loading message until users are fetched
            ) : (
                users.map((userId, i) => (
                    <UserCard
                        key={i}
                        userId={userId}
                    />
                ))
            )}
        </Stack>
    );
};

export default UsersLayout;
