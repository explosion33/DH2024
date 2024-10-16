import { useState, useEffect } from "react";
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardMedia from '@mui/material/CardMedia';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import CardActions from '@mui/material/CardActions';
import ContactPaper from './ContactPaper';

const UserCard = ({ userId }) => {
    console.log("UserCard");

    const [open, setOpen] = useState(false);

    const handleOpen = () => setOpen(true);

    let response = {};

    const getInfo = () =>{
        console.log("getInfoCalled");
        fetch('https://e2.armstronglabs.net/api/info/'+userId, {
            method: "GET",
            headers: {
            "Content-Type": "application/json",
            "Accept": "application/json"
        }}).then(resp => {console.log(resp); resp.json().then(res => {response = res; console.log(res);})});
    }
     //useEffect(() => {
         getInfo();
         console.log("getInfoDone");
     //}, []);
    return (
        <Card sx={{
            width: 300,            // Sets a fixed width for all cards
            height: 300,           // Sets a fixed height for all cards
        }}>
            <CardMedia
                component="img"
                height="140"
                image="https://mui.com/static/images/cards/contemplative-reptile.jpg"
                alt={`${response.first} ${response.last}`}
                sx={{
                    maxWidth: '100%',      // Sets max width relative to the Card component
                    maxHeight: 140,        // Sets the max height
                    objectFit: 'cover'     // Ensures the image scales to fit within the dimensions without distortion
                }}
            />
            <CardContent>
                <Typography gutterBottom variant="h5" component="div">
                    {response.first} {response.last}
                </Typography>
                <Typography variant="body2" sx={{ color: 'text.secondary' }}>
                    Looking for: {response.wants[0] || "No Wants"}
                    <br />
                    Offering: {response.skills[0] || "No skills listed"}
                </Typography>
            </CardContent>
            <CardActions>
                <Button size="small">Share</Button>
                <Button size="small" onClick={() => handleOpen()}>Lets Connect!</Button>
            </CardActions>
            {open && <ContactPaper open={open} setOpen={setOpen} user={user} />}
        </Card>
    )
};


export default UserCard;

