use clap::{arg, ArgAction, Command};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("oi")
        .about("Get AI powered short answers to questions")
        .version("0.1.0")
        .arg(
            arg!(--configure "Opens the configuration file in your $EDITOR")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(arg!([question] "The question to ask the AI").required(false))
        .get_matches();

    if matches.get_flag("configure") {
        println!("Opening configuration...");
    } else if let Some(question) = matches.get_one::<String>("question") {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found.");

        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("Shell"));
        let os = env::consts::OS;

        let system_message = format!("You are an expert {} administrator helping a user who is using the command line. The user uses {} and {}. Provide your answer in two sentences or fewer, mindful of the fact that the answer will be presented over the command line. If the question is about a specific command to be run, provide that command in its full as the response, using a maximum of one sentence to explain the command or why that specific command.", os, shell, os);

        let messages = json!([
            {
                "role": "system",
                "content": system_message
            },
            {
                "role": "user",
                "content": question
            }
        ]);

        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages
        });

        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let response_body: serde_json::Value = res.json().await?;
        let response_message = response_body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .trim();

        println!("{}", response_message);
    }

    Ok(())
}
