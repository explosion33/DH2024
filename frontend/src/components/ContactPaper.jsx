import React, { useState } from 'react';
import { Modal, Paper, Typography, Button, Box } from '@mui/material';

const ContactPopup = ({open, setOpen, user}) => {


    return (
        <>
            <Modal
                open={open}
                onClose={() => setOpen(false)}
                aria-labelledby="contact-modal-title"
                aria-describedby="contact-modal-description"
                sx={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    backdropFilter: 'blur(4px)', // Adds a slight blur to the background
                }}
            >
                <Paper
                    sx={{
                        padding: 4,
                        maxWidth: 400,
                        width: '100%',
                        outline: 'none',
                    }}
                >
                    <Typography variant="h6" id="contact-modal-title" gutterBottom>
                        {user.first} {user.last}'s Contact
                    </Typography>
                    <Typography variant="body1" id="contact-modal-description">
                        Phone: {user.contact[0]}
                        <br />
                        Email: {user.contact[1]}
                    </Typography>
                    {/* Add form fields, buttons, etc., inside ContactPaper */}
                    <Button onClick={() => setOpen(false)} variant="contained" color="secondary" sx={{ mt: 2 }}>
                        Close
                    </Button>
                </Paper>
            </Modal>
        </>
    );
};

export default ContactPopup;