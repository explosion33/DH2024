import UsersLayout from "../components/UsersLayout";

const PeopleView = () => {

    return (

            <UsersLayout
                display="flex"
                justifyContent="center"       // Centers items horizontally
                alignItems="center"           // Centers items vertically (optional)
                flexWrap="wrap"               // Allows items to wrap to the next line if needed
                gap={2}                       // Adds spacing between items
                sx={{ maxWidth: '100%', margin: '2 auto' }} // Centers container itself and limits width
            />
    );
}

export default PeopleView;