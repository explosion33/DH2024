import requests
import sseclient
import json
import logging
from typing import Generator, Union, Dict, Any
import re

def make_perplexity_api_call(
    api_key: str, 
    model: str, 
    user_message: str, 
    stream: bool = False,
    temperature: float = 0.15,
    max_tokens: int = 2000
) -> Union[Dict[str, Any], Generator[str, None, None]]:
    url = "https://api.perplexity.ai/chat/completions"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }
    payload = {
        "model": model,
        "messages": [{"role": "user", "content": user_message}],
        "temperature": temperature,
        "max_tokens": max_tokens,
        "stream": stream
    }
    try:
        return _normal_response(url, payload, headers)
    except requests.exceptions.RequestException as e:
        logging.error(f"Request failed: {e}")
        return {"error": str(e)}

def _normal_response(
    url: str, 
    payload: dict, 
    headers: dict
) -> Dict[str, Any]:
    try:
        response = requests.post(url, json=payload, headers=headers)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.RequestException as e:
        logging.error(f"Normal request failed: {e}")
        return {"error": f"Request error: {str(e)}"}

# Usage Example
api_key = "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"

# Define the user message that you want to send to the API
user_message = """
I have a list of users, where each user has two lists: one containing skills they know and another containing skills they want to learn. I want to match users based on both exact and similar skills. Related skills should have non-zero match percentages, even if they are not identical.

For example, if one user knows 'Chess' and another wants to learn 'Checkers', they should receive a match percentage of around 20% because they are somewhat similar. Similarly, 'Football' and 'Rugby' should also be matched with a lenient similarity score, perhaps around 20% as well. Exact matches should still receive a 100% score, and unrelated skills should score as 0%.

Additionally, the AI must return a match for every user in the input, even if their match score is 0. An empty matches list should not be allowed.

For each user, I need to find all other users whose known skills are related to or match the skills the user wants to learn and provide a match percentage based on similarity.

when evaluating a match examine all entries in both list and record the score of the most similar skills. Be leniant with how similar skills are


Here is an example:

Input:

[
    {
        "uid": "3",
        "knows": ["Sewing", "Metal Art"],
        "wants": ["Football", "Chess"]
    },
    {
        "uid": "4",
        "knows": ["checkers", "rugby"],
        "wants": ["badminton", "bike"]
    }
]

Expected Output:

[
    {
        "uid": "3",
        "matches": [("4", 20.0)]
    },
    {
        "uid": "4",
        "matches": [("3", 20.0)]
    }
]

Please return the matched users and their similarity percentages based on relatedness of their skills, and ensure that each user has a match entry even if the score is 0. Do not provide any code—just the matching results in this format.
ensure if there are X total users each user has X-1 matches.
evaluate all skills in the list and choose the closest skill match, only recoring that in the final output
provide a brief explanation for what skill you used to generate a score and why the value was chosen, before outputing the final results

Below is the data I want to generate matches for

[
	{
    	uid: "1",
    	knows: ["programming", "woodworking"],
    	wants: ["welding", "cooking"]
	},
	{
    	uid: "2",
    	knows: ["welding", "Baking"],
    	wants: ["Game Design", "Construction"]
	},
    {
    	uid: "3",
    	knows: ["Sewing", "Metal Art"],
    	wants: ["Football", "Chess"]
	},
    {
        uid: "4",
        knows: ["checkers", "rugby"]
        wants: ["badminton", "bike"]
    },
]
"""

user_message = """
I have a list of users containing the skills they have and the skills the want to learn.

For each user, compare each skill to every other users wants and generate a percentage that
reflects how similar they are to each other. An exact match is 100% a completely different match
is 0% and matches can fall in between. For Example, programming and rocketry may be 50% due to the
fact that rockets often contain some programming work in their avionics.

DO NOT RETURN STEPS OR CODE. Instead return lists of percentages and uids based off of your analysis.
For example, the output should look as follows.

{
    "1": {
        "2": {
            "skill 1": {
                "want 1": 20.0,
                "want 2": 95.0
            },
            "skill 2": {
                ...
            }
        },
        ...
    },
    "2": {
        ...
    }
}


    
Here is my data:

[
	{
    	uid: "1",
    	knows: ["programming", "woodworking"],
    	wants: ["welding", "cooking"]
	},
	{
    	uid: "2",
    	knows: ["welding", "Baking"],
    	wants: ["Game Design", "Construction"]
	},
    {
    	uid: "3",
    	knows: ["Sewing", "Metal Art"],
    	wants: ["Football", "Chess"]
	},
    {
        uid: "4",
        knows: ["checkers", "rugby"]
        wants: ["badminton", "bike"]
    },
]
"""

# Get normal response
normal_response = make_perplexity_api_call(
    api_key,
    "llama-3.1-sonar-large-128k-online",
    user_message,
    stream=False
)

x = json.dumps(normal_response, indent=2)

x = normal_response["choices"][0]["message"]["content"]

# Print the entire response for debugging

#print(x)

x = re.search("```(.*\n)*?({(.*\n)*})(.*\n)*?```", x)

x = x.group(2)

x = re.sub("//.*", "", x)


print(x)

x = json.loads(str(x))
print(x)



out = []

for uid in x.keys():
    matches = []

    for match in x[uid]:
        max = 0
        for skill in x[uid][match]:
            for want in x[uid][match][skill]:
                if (x[uid][match][skill][want] > max):
                    max = x[uid][match][skill][want]
        matches.append([match, max])

    out.append({
        "uid": uid,
        "matches": matches
    })

print(out)

#x = re.search("```.*\n(\[(.*\n)*\])\n```", x)

#print(x)
