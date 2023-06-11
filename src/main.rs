use clap::{arg, Command, ArgAction};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use std::env;
use colored::*;

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
        .arg(
            arg!(--debug "Print debug information to the console")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            arg!([question] "The question to ask the AI")
                .required(false),
        )
        .get_matches();

    let is_debug = matches.get_flag("debug");

    if matches.get_flag("configure") {
        println!("Opening configuration...");
    } else if let Some(question) = matches.get_one::<String>("question") {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found.");

        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("Shell"));
        let os = env::consts::OS;

        if is_debug {
            println!("{}", format!("OS: {}\nShell: {}\nOpenAI Key: {}", os, shell, api_key).yellow());
        }

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
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key))?);

        let res = client.post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await;

        match res {
            Ok(response) => {
                let response_body: serde_json::Value = response.json().await?;
                if let Some(response_message) = response_body["choices"][0]["message"]["content"].as_str() {
                    println!("{}", response_message.trim());
                } else {
                    println!("{}", "OpenAI response not as expected.".red());
                    return Err("OpenAI response not as expected".into());
                }
            },
            Err(_) => {
                println!("{}", "OpenAI request failed. Please try again.".red());
                return Err("OpenAI request failed".into());
            },
        }
    }

    Ok(())
}

