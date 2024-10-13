/* eslint react/prop-types: 0 */
const SkillsMastered = ( {setStage, setSkills} ) => {
    return (
        <div>
            <header>
                <h1 className="text-3xl font-bold underline">
                    What skills do you have to share?
                </h1>
            </header>
            <main className="question"><center>
                <textarea onChange={(e) => {setSkills(e.target.value)}}/>
                <button onClick={() => {setStage(3)}}>Continue →</button>
            </center></main>
        </div>
    )
}

export default SkillsMastered;