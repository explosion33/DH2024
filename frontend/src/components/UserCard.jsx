import React from "react";
import Card from '@mui/material/Card';
import CardContent from '@mui/material/CardContent';
import CardMedia from '@mui/material/CardMedia';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import CardActions from '@mui/material/CardActions';

const UserCard = () => {

    return (
            <Card sx={{ maxWidth: 345 }}>
                <CardMedia
                    component="img"
                    height="140"
                    image="image"
                    alt=""
                />
                <CardContent>
                    <Typography gutterBottom variant="h5" component="div">
                        "Name"
                    </Typography>
                    <Typography variant="body2" sx={{ color: 'text.secondary' }}>
                        Looking for:
                        Offering:
                    </Typography>
                </CardContent>
                <CardActions>
                    <Button size="small">Share</Button>
                    <Button size="small">Learn More</Button>
                </CardActions>
            </Card>
        )
};


export default UserCard;

