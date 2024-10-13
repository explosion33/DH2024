/* eslint react/prop-types: 0 */
const SkillsMastered = ( {setStage, setSkills} ) => {
    return (
        <div>
            <h1 className="text-3xl font-bold underline">
                What skills do you have to share?
            </h1>
            <textarea onChange={(e) => {setSkills(e.target.value)}}/>
            <button onClick={() => {setStage(3)}}>Continue →</button>
        </div>
    )
}

export default SkillsMastered;