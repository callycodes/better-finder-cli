// main.rs

mod location_config;
use location_config::LOCATIONS;

mod session;
use session::{Session, ApiResponse, Data};

use serde_json::from_str;

use colored::*;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    println!("{}", "Welcome, let's find you an activity".red());

    let mut date = String::new();
    println!("{}", "Enter a date, format YYYY-MM-DD: ".green());
    io::stdin().read_line(&mut date).unwrap();
    let date: String = date.trim().parse().expect("Please enter a date.");

    println!("{}", "Here are your activity configs...".green());
    for (i, location) in LOCATIONS.iter().enumerate() {
        println!("{}. {} ({})", i + 1, location.location_tag, location.activity_tag);
    }

    let mut choice = String::new();
    println!("{}", "Select a location by number: ".green());
    io::stdin().read_line(&mut choice).unwrap();
    let choice: usize = choice.trim().parse().expect("Please enter a number.");

    if choice > 0 && choice <= LOCATIONS.len() {
        let location = &LOCATIONS[choice - 1];
        println!("{}", "Searching venues:".green());
        for venue in location.venue_tags {
            println!("- {}", venue.blue());

            let client = Client::new();
    let url = format!("https://better-admin.org.uk/api/activities/venue/{}/activity/{}/times?date={}", venue, location.activity_tag, date);

    let mut headers_map = HeaderMap::new();

    let referer_header_value = format!("https://bookings.better.org.uk/location/{}/{}/{}/by-time", venue, location.activity_tag, date);

    let headers = [
        ("accept", "application/json"),
        ("accept-language", "en-GB,en-US;q=0.9,en;q=0.8"),
        ("origin", "https://bookings.better.org.uk"),
        ("sec-ch-ua", "\"Chromium\";v=\"122\", \"Not(A:Brand\";v=\"24\", \"Google Chrome\";v=\"122\""),
        ("sec-ch-ua-mobile", "?1"),
        ("sec-ch-ua-platform", "\"Android\""),
        ("sec-fetch-dest", "empty"),
        ("sec-fetch-mode", "cors"),
        ("sec-fetch-site", "cross-site"),
        ("user-agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Mobile Safari/537.36"),
    ];

    for (key, value) in headers {
        let header_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
        let header_value = HeaderValue::from_str(value).unwrap();
        headers_map.insert(header_name, header_value);
    }

    let referer_header = HeaderValue::from_str(&referer_header_value).expect("Invalid header value");
    headers_map.insert(HeaderName::from_static("referer"), referer_header);


    let response = client.get(url)
        .headers(headers_map)
        .send()?
        .text()?;


        let api_response: ApiResponse = serde_json::from_str(&response)?;

match api_response.data {

    Data::Array(sessions) if sessions.is_empty() => {
        println!("{}", "No sessions".yellow());
    },

    Data::Array(sessions) => {
        // Handle the case where `data` is an array
        for session in sessions.iter().filter(|s| s.spaces > 1) {
            println!("{} -> {} - {} available", 
                session.starts_at.format_12_hour.yellow(), 
                session.ends_at.format_12_hour.yellow(), 
                session.spaces.to_string().green());
        }
    },
    Data::Object(session_map) if session_map.is_empty() => {
        println!("{}", "No sessions".yellow());
    },
    Data::Object(session_map) => {
        // Handle the case where `data` is an object
        for (key, session) in session_map.iter().filter(|(_, s)| s.spaces > 1) {
            println!("{} -> {} - {} available", 
                session.starts_at.format_12_hour.yellow(), 
                session.ends_at.format_12_hour.yellow(), 
                session.spaces.to_string().green());
        }
    },
}


        }
    } else {
        println!("Invalid choice.");
    }


    Ok(())
}
