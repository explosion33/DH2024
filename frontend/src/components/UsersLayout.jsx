import { React, useState, useEffect } from 'react';
import Stack from '@mui/material/Stack';
import UserCard from './UserCard';

const UsersLayout = () => {

    const getUsers = async () => {
        const response = await fetch('/');
        const usersJson = await response.json();
        let users = [];

        for (let user in usersJson){
            users.push(user);
        }
    } 

    useEffect(() => {
        getUsers();
    }, []);

    return (
        <Stack direction="row" spacing={2}>
            {users.map((user, i) => 
                <UserCard 
                key={i}
                user={user}
                />)
            }
        </Stack>
    )
};

export default UsersLayout;