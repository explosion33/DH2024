/* eslint react/prop-types: 0 */
const SkillsNeeded = ({setStage}) => {
    return (
        <div>
            <h1 className="text-3xl font-bold underline">
                What skills are you interested in learning?
            </h1>
            <textarea />
            <button onClick={() => setStage(2)}>Continue →</button>
        </div>
    )
}

export default SkillsNeeded;