
pub mod short_access_token;
use crate::short_access_token::ShortAccessToken;

mod refresh_token;
use crate::refresh_token::RefreshToken;

//This mod should just make requests and return the info
mod strava_api;
use crate::strava_api::{request_new_access_tokens, request_activity_stream};

mod activity_map;
use crate::activity_map::ActivityMap;

use std::io::{stdout, stdin, Read, Write};
use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use bincode;
use strava_api::request_specific_activity;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::str;
use chrono::prelude::*;

// pub fn termporary_refresh_save(rt: RefreshToken){
//     rt.save_refresh_token();
// }

//Write token data, either refresh token or short access token, to their specified file
pub fn write_to_file(file_path: &Path, data: Vec<u8>) {
    let mut file = OpenOptions::new().write(true).open(file_path).unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create(file_path).unwrap_or_else(|err| {
                panic!("Problem creating file: {:?}", err);
            })
        } else {
            panic!("Problem opening file: {:?}", err);
        }
    });

    file.write_all(&data).unwrap_or_else(|err| {
        panic!("Problem writing serialized token to file: {:?}", err);
    });
}

//Get current short access token. Data is serialized and saved in local file
pub fn get_short_access_token() -> ShortAccessToken{
    let path = Path::new("short_access_token_data.txt");
    let display = path.display();

    let mut file = File::open(&path).unwrap_or_else(|err| {
        panic!("problem opening file: {:?} Error {:?}", display, err);
    });

    let mut buf: Vec<u8> = Vec::new();

    file.read_to_end(&mut buf).unwrap_or_else(|err| {
        panic!("problem reading into buffer {:?}", err);
    });

    let deserialized_token: ShortAccessToken = bincode::deserialize(&buf).unwrap_or_else(|err| {
        panic!("problem deserializing buffer {:?}", err);
    });

    println!("Short access token: {:?}", deserialized_token);

    if deserialized_token.check_if_expired(){
        println!("token expired. New short access token");
        refresh_access_token();
        let new_sat = get_short_access_token();
        new_sat
    } else {
        deserialized_token
    }
}

//Get the RefreshToken that is serialized and saved in local file
pub fn get_last_refresh_token() -> RefreshToken{
    let path = Path::new("refresh_token_data.txt");
    let display = path.display();

    //Open file
    let mut file = File::open(&path).unwrap_or_else(|err| {
        panic!("problem opening file: {}, Error: {:?}", display, err);
    });

    let mut buf: Vec<u8> = Vec::new();

    //Read data from local file into byte buffer
    file.read_to_end(&mut buf).unwrap_or_else(|err| {
        panic!("problem reading into buffer {:?}", err);
    });

    //Deserialize data: binary -> RefreshToken
    let deserialized_token: RefreshToken = bincode::deserialize(&buf).unwrap_or_else(|err| {
        panic!("problem deserializing buffer {:?}", err);
    });

    deserialized_token
}

//When short access token expires, use this to get a new one
//HTTP POST to strava, sends my refresh token and recieve a new short access token and a new refresh token
pub fn refresh_access_token(){

    let refresh_token = get_last_refresh_token();
    let contents_string = request_new_access_tokens(&refresh_token);
    let parsed: Value = read_json(str::from_utf8(&contents_string).unwrap());

    //Strava will occasionally give a new refresh token, and we need to replace our current one with the new one.
    if parsed["refresh_token"] != refresh_token.token_string {
        let new_refresh_token = RefreshToken::build(parsed["refresh_token"].to_string());
        new_refresh_token.save_refresh_token();
        println!("New REFRESH TOKEN {}", new_refresh_token.token_string);
    }

    let sat = ShortAccessToken::build(
        remove_extra_characters(&parsed["access_token"].to_string()),
        remove_extra_characters(&parsed["expires_at"].to_string()),
    );

    sat.save_short_access_token();

}

pub fn get_activity_stream(){
    let activity_id = "7955070771";
    let keys = vec!["time".to_string(), "latlng".to_string()];
    let contents = request_activity_stream(activity_id.to_string(), keys);
    let json: Value = serde_json::from_str(str::from_utf8(&contents).unwrap()).unwrap();

    // file.write_all(&contents).unwrap_or_else(|err| {
    //     panic!("Problem writing GPX data to file: {:?}", err);
    // });

    let activity_map = ActivityMap::build_from_json(activity_id.to_owned(), json);

    // println!("{}", activity_map.)
    activity_map.get_location_points();
}

//using this to remove the (\) and (") from the parsed json data, might be unnecessary idk
fn remove_extra_characters(str: &str) -> String {
    let result = str.replace(&['\\', '\"'][..], "");
    println!("remove char results: {}", result);
    result
}

fn read_json(raw_json:&str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    parsed
}





//this should handle requests to google and return data
mod google_map_api{

}

pub fn get_sp(){
    request_specific_activity("7955070771".to_string());
}