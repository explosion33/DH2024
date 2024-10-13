import { useState } from "react";
import Introduction from "../components/Introduction";
import SkillsNeeded from "../components/SkillsNeeded";
import SkillsMastered from "../components/SkillsMastered";

const Onboarding = () => {
    let [stage, setStage] = useState(0);

    return (
        (stage == 0) ? <Introduction setStage={setStage} /> : (stage == 1) ? <SkillsNeeded setStage={setStage}  /> : <SkillsMastered/>
    )
};

export default Onboarding;