import requests
import json

# API endpoint and key
url = "https://api.perplexity.ai/chat/completions"
api_key = "<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>"

# Payload for the API request
payload = {
    "model": "llama-3.1-sonar-small-128k-online",
    "messages": [
        {
            "role": "system",
            "content": ""    # Optional system message
        },
        {
            "role": "user",
            "content": "What is the compatibility of different systems?"  # Example query
        }
    ],
    "max_tokens": "Optional",
    "temperature": 0.2,
    "top_p": 0.9,
    "return_citations": False,
    "search_domain_filter": ["perplexity.ai"],
    "return_images": False,
    "return_related_questions": False,
    "search_recency_filter": "month",
    "top_k": 0,
    "stream": False,
    "presence_penalty": 0,
    "frequency_penalty": 1
}

# Headers for the API request (with valid API key)
headers = {
    "Authorization": f"Bearer {api_key}",
    "Content-Type": "application/json"
}

# Send the POST request to the Perplexity AI API
response = requests.post(url, json=payload, headers=headers)

# Check if the request was successful
if response.status_code == 200:
    # Parse the JSON response
    data = response.json()
    
    # Specify the threshold value (for example, compatibility score)
    threshold = 50  
    
    # Assuming the JSON contains an array of results with a 'score' or 'compatibility' field
    if "results" in data:  # Ensure 'results' key exists in the response
        filtered_results = [
            result for result in data["results"]
            if result.get("score", 0) > threshold  # Check if 'score' is above threshold
        ]
        
        # Output the filtered results
        print(f"Results with score above {threshold}:")
        for result in filtered_results:
            print(json.dumps(result, indent=4))  # Pretty print the filtered result
    else:
        print("No 'results' key in the response JSON.")
else:
    print(f"Request failed with status code {response.status_code}")
    print(f"Response: {response.text}")
