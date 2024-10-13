import requests
import sseclient
import json
import logging
from typing import Generator, Union, Dict, Any

def make_perplexity_api_call(
    api_key: str, 
    model: str, 
    user_message: str, 
    stream: bool = False,
    temperature: float = 0.2,
    max_tokens: int = 1000
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
        if stream:
            return _stream_response(url, payload, headers)
        else:
            return _normal_response(url, payload, headers)
    except requests.exceptions.RequestException as e:
        logging.error(f"Request failed: {e}")
        return {"error": str(e)}

def _stream_response(
    url: str, 
    payload: dict, 
    headers: dict
) -> Generator[str, None, None]:
    try:
        with requests.post(url, json=payload, headers=headers, stream=True) as response:
            response.raise_for_status()
            for event in sseclient.SSEClient(response).events():
                if event.data == '[DONE]':
                    break
                try:
                    content = json.loads(event.data)['choices'][0]['delta'].get('content', '')
                    if content:
                        yield content
                except json.JSONDecodeError:
                    logging.error(f"Failed to decode JSON: {event.data}")
    except requests.exceptions.RequestException as e:
        logging.error(f"Stream request failed: {e}")
        yield f"Error: {str(e)}"

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
Here are some users with their wants and knows:
user1: [['user2', 'machine learning', 'machine learning', 0], ['user3', 'game design', 'game design', 0]]
user2: [['user1', 'SEO optimization', 'SEO optimization', 0], ['user3', 'virtual reality development', 'virtual reality development', 0]]
user3: [['user1', 'web development', 'web development', 0], ['user2', 'social media management', 'social media management', 0]]

Please provide compatibility scores based on these wants and knows. Examples of scoring:
- Math and Geometry: 0.9
- Swimming and Running: 0.7
- Painting and Baseball: 0.0
- SEO optimization and digital marketing: 0.8
- Game design and virtual reality development: 0.6
- Web development and social media management: 0.5
"""

# Get normal response
normal_response = make_perplexity_api_call(
    api_key,
    "llama-3.1-sonar-small-128k-online",
    user_message,
    stream=False
)

print(normal_response)

# Print the entire response for debugging
print("Full API Response:", json.dumps(normal_response, indent=2))

# Check if the response contains 'choices'
if 'choices' in normal_response:
    print("Parsed Compatibility Data:")
    for user in normal_response['choices']:
        # Here we check if 'userID' exists in the user dictionary
        if 'userID' in user:
            userID = user['userID']
            compatibilities = user['compatibilities']
            print(f"{userID}: {compatibilities}")
        else:
            print("No userID found in:", user)
else:
    print("Error in response:", normal_response.get("error", "Unknown error"))
