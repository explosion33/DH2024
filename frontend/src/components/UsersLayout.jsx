import { React, useState, useEffect } from 'react';
import Stack from '@mui/material/Stack';
import UserCard from './UserCard';
import { useAuth0 } from "@auth0/auth0-react";
import { fakeUsers } from './FakeUser'


const UsersLayout = () => {

    // const { user, isAuthenticated, isLoading } = useAuth0();
    let users = [];

    // const getUsers = () => {

    //     users = fakeUsers;
    //     console.log(users);

    //     // const response = await fetch('http://127.0.0.1:80/users/100', {
    //     //     method: "GET",
    //     //     headers: {
    //     //       "Content-Type": "application/json",
    //     //       "Accept": "application/json"
    //     //     },
    //     //   });

    //     //   const userJson = await response.json();
    //     // // const actualResults = await response.text();
    //     // console.log("user json:", userJson);

    //     // for (let user in users) {
    //     //     users.push(user);
    //     // }
    // }

    // useEffect(() => {
    //     getUsers();
    // }, []);

    users = fakeUsers;
    console.log(users);


    return (
        <>
            <Stack spacing={3}
                direction="row"
                useFlexGap
                justifyContent="center"       // Centers items horizontally
                alignItems="center"  
                sx={{ flexWrap: 'wrap', margin: '5% auto' }}>
                {users.map((user, i) =>
                    <UserCard
                        key={i}
                        user={user}
                    />)
                }
            </Stack>
        </>
    )
};

export default UsersLayout;