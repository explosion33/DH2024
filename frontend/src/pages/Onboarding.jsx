import { useState } from "react";
import { useNavigate } from 'react-router-dom';
import { useAuth0 } from "@auth0/auth0-react";
import Introduction from "../components/Introduction";
import SkillsNeeded from "../components/SkillsNeeded";
import SkillsMastered from "../components/SkillsMastered";

const Onboarding = () => {
    let [stage, setStage] = useState(0);
    let [contact, setContact] = useState("");
    let [skills, setSkills] = useState("");
    let [wants, setWants] = useState("");
    const { loginWithRedirect, user } = useAuth0();
    const navigate = useNavigate();

    return (
        user == null ? loginWithRedirect() : 
        (stage == 0) ? <Introduction setStage={setStage} setContact={setContact} /> : 
        (stage == 1) ? <SkillsNeeded setStage={setStage} setWants={setWants}  /> : 
        (stage == 2) ? <SkillsMastered setStage={setStage} setSkills={setSkills} /> : 
        fetch("http://localhost:8081/info", {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({uid: user?.sub, first: user?.given_name, last: user?.family_name, contact: contact.split("\n"), skills: skills.split(","), wants: wants.split(",")}),
        }).then((response) => {
            if (response.status === 200) {
                navigate("/people");
            } else {
                setStage(0);
            }
        })
    )
};

export default Onboarding;