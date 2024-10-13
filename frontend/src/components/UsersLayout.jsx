import { useState, useEffect } from "react";
import Stack from '@mui/material/Stack';
import UserCard from './UserCard';
import { useAuth0 } from "@auth0/auth0-react";


const UsersLayout = () => {

    const { user } = useAuth0();
    let users = [];

    console.log("UsersLayout");
    const getUsers = async () => {
        console.log("getUsers")

          fetch('https://e2.armstronglabs.net/api/matches/'+user?.sub+'/100', {
              method: "GET",
              headers: {
                "Content-Type": "application/json",
                "Accept": "application/json"
              },
            }).then(response => response.json().then(response => {
                console.log("Filling Users")
                console.log(response);
                for (let user of response.matches) {
                    users.push(user.uid);
                }
            }));

     }

     useEffect(() => {
         getUsers();

         console.log("users", users);
     }, []);

    return (
        <>
            <Stack spacing={3}
                direction="row"
                useFlexGap
                justifyContent="center"       // Centers items horizontally
                alignItems="center"  
                sx={{ flexWrap: 'wrap', margin: '5% auto' }}>
                {users.map((userId, i) =>
                    <UserCard
                        key={i}
                        userId={userId}
                    />)
                }
            </Stack>
        </>
    )
};

export default UsersLayout;