mod api;
mod player;

use std::error::Error;

use log::{error, info};
use std::time::SystemTime;

use api::ApiResponse;
use player::Player;

use csv::ReaderBuilder;
use rocket::fs::{relative, FileServer};
use rocket::http::{ContentType, Header, Status};
use rocket::serde::json::serde_json;

#[macro_use]
extern crate rocket;

#[get("/player/<nick>")]
fn get_player(nick: String) -> ApiResponse {
    info!("GET Request: /player/{}", nick);
    let response = || -> Result<ApiResponse, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .delimiter(b'#')
            .quoting(true)
            .has_headers(false)
            .from_path("dataset/hltv_playerStats-complete.txt")?;
        let mut body = "".to_string();
        for result in reader.deserialize() {
            let record: Player = result?;
            if record.nick == nick {
                body = serde_json::to_string(&record)?;
                break;
            }
        }
        info!("RESPONSE /player/<nick>: {}", body);
        Ok(ApiResponse {
            body: body,
            content_type: ContentType::JSON,
            status: Header::new("Status", Status::Ok.to_string()),
        })
    };

    let player = response();

    if let Err(e) = player {
        error!("ERROR IN /player/<nick>: {}", e.to_string());
        return ApiResponse {
            body: e.to_string(),
            content_type: ContentType::Text,
            status: Header::new("Status", Status::InternalServerError.to_string()),
        };
    } else if let Ok(p) = player {
        return p;
    } else {
        return ApiResponse {
            body: "Something went wrong".to_string(),
            content_type: ContentType::Text,
            status: Header::new("Status", Status::InternalServerError.to_string()),
        };
    }
}

#[get("/players")]
fn get_all_players() -> ApiResponse {
    info!("GET Request: /players");
    let response = || -> Result<ApiResponse, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .delimiter(b'#')
            .quoting(true)
            .has_headers(false)
            .from_path("dataset/hltv_playerStats-complete.txt")?;
        let mut players: Vec<Player> = vec![];
        for result in reader.deserialize() {
            let record: Player = result?;
            players.push(record);
        }
        let body = serde_json::to_string(&players)?;
        info!("RESPONSE SUCCEEDED /players");
        Ok(ApiResponse {
            body: body,
            content_type: ContentType::JSON,
            status: Header::new("Status", Status::Ok.to_string()),
        })
    };

    let player = response();

    if let Err(e) = player {
        error!("ERROR IN /players: {}", e.to_string());
        return ApiResponse {
            body: e.to_string(),
            content_type: ContentType::Text,
            status: Header::new("Status", Status::InternalServerError.to_string()),
        };
    } else if let Ok(p) = player {
        return p;
    } else {
        return ApiResponse {
            body: "Something went wrong".to_string(),
            content_type: ContentType::Text,
            status: Header::new("Status", Status::InternalServerError.to_string()),
        };
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    let log_filepath: String = format!(
        "{}_output.log",
        humantime::format_rfc3339_seconds(SystemTime::now())
            .to_string()
            .replace(":", "")
    );

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_filepath)?)
        .apply()?;
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("/../frontend/build")))
        .mount("/api", routes![get_player, get_all_players])
        .launch()
        .await?;

    Ok(())
}
