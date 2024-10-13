import { useAuth0 } from "@auth0/auth0-react";
/* eslint react/prop-types: 0 */
const Introduction = ({setStage}) => {
    const { user } = useAuth0();
    return (
        <div>
            <h1 className="text-3xl font-bold underline">
                Welcome, {user.name}! How should people contact you?
            </h1>
            <ul id="contactList">
                <li><input placeholder="Email, phone number, etc." type="text" /> <button onClick={this.parentNode.remove()}>-</button></li>
            </ul>
            <button onClick={
                document.getElementById("contactList").append("<li><input placeholder='Email, phone number, etc.' type='text' /> <button>-</button></li>") }>+</button>
            <button onClick={() => setStage(1)}>Continue →</button>
        </div>
    )
}

export default Introduction