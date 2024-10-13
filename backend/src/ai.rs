use rocket::serde::json::serde_json::{self, json, Value};
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::routes::{Match, Matches, User};

const PROMPT: &str = "I have a list of users containing the skills they have and the skills the want to learn.

    For each user, compare each skill to every other users wants and generate a percentage that
    reflects how similar they are to each other. An exact match is 100% a completely different match
    is 0% and matches can fall in between. For Example, programming and rocketry may be 50% due to the
    fact that rockets often contain some programming work in their avionics.

    DO NOT RETURN STEPS OR CODE. Instead return lists of percentages and uids based off of your analysis.
    For example, the output should look as follows.

    DO NOT RETURN AN EXAMPLE STRUCUTRE. RETURN THE FULL EXPECTED OUTPUT

    {
        \"1\": {
            \"2\": {
                \"skill 1\": {
                    \"want 1\": 20.0,
                    \"want 2\": 95.0
                },
                \"skill 2\": {
                    ...
                }
            },
            ...
        },
        \"2\": {
            ...
        }
    }
    
    Here is my data:

";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Query_Data {
    uid: String,
    knows: Vec<String>,
    wants: Vec<String>,
}

pub fn parse_users(users: Vec<User>) -> Vec<Query_Data> {
    let mut out = Vec::new();

    for user in users {
        out.push(Query_Data {
            uid: user.uid,
            knows: user.skills,
            wants: user.wants,
        });
    }


    out
}

pub async fn call_and_parse(users: Vec<User>) -> Result<Vec<Matches>, ()> {
    let body = format!("{}{}", PROMPT, serde_json::to_string(&users).unwrap());

    let mut payload = json!({
        "model": "llama-3.1-sonar-large-128k-online",
        "messages": [{"role": "user", "content": body}],
        "temperature": 0.15f32,
        "max_tokens": 5000,
        "stream": false
    }).to_string();
    println!("body: {}", payload);

    let client = reqwest::Client::new();
    let mut response = client
        .post("https://api.perplexity.ai/chat/completions")
        .header("Authorization", format!("Bearer {}", "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"))
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await
        .expect("response fail")
        .text()
        .await
        .expect("aaaaaah");


    response = response.replace("\\n", "\n");
    response = response.replace("\\\"", "\"");
    

    println!("raw: {}", response);

    //println!("res: {}", response.as_str());

    let re = Regex::new(r"```(.*\n)*?(\{(.*\n)*\})(.*\n)*?```").expect("regegx fail");
    let captures = re
        .captures(response.as_str())
        .ok_or("Failed to extract JSON from response").expect("error");
    let json_str = captures.get(2).ok_or("Failed to capture JSON content").expect("error assa").as_str();

    let re = Regex::new(r"//.*").unwrap();
    let json_str = re.replace_all(json_str, "");

    println!("res: {}", json_str);

    let parsed_json: Value = match serde_json::from_str(&json_str) {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR | {:?}", e);
            return Err(());
        }
    };

    let mut output: Vec<Matches> = Vec::new();

    if let Some(users) = parsed_json.as_object() {
        for (uid, matches) in users {
            let mut user_matches: Vec<Match> = Vec::new();

            if let Some(matches_obj) = matches.as_object() {
                for (match_uid, skills) in matches_obj {
                    let mut max_score = 0f32;

                    if let Some(skills_obj) = skills.as_object() {
                        for (skill, wants) in skills_obj {
                            if let Some(wants_obj) = wants.as_object() {
                                for (_, score) in wants_obj {
                                    let score_value = score.as_f64().unwrap_or(0.0) as f32;
                                    if score_value > max_score {
                                        max_score = score_value;
                                    }
                                }
                            }
                        }
                    }

                    user_matches.push(Match {
                        uid: match_uid.to_owned(),
                        percent: max_score,
                    });
                }
            }

            output.push(Matches {
                uid: uid.clone(),
                matches: user_matches,
            });
        }
    }

    Ok(output)
}

