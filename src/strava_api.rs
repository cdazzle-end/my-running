use std::io::{stdout, stdin, Read, Write};
use chrono::format::format;
use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use bincode;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::str;
use chrono::prelude::*;

use crate::get_last_refresh_token;
use crate::get_short_access_token;
use crate::refresh_token::RefreshToken;
use crate::short_access_token::ShortAccessToken;
use crate::read_json;
use crate::remove_extra_characters;

static CLIENT_ID: &str = "94993";
static CLIENT_SECRET: &str = "a5ce4ce75a78b46db119559a85e12833e390b8f6";

//Byte buffer that will be wrapped in the Easy handler and collect data recieved from HTTP requests
pub struct Collector(pub Vec<u8>);

//Required trait to implement when creating an Easy handler, writes data into the byte buffer
impl Handler for Collector{
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        // stdout().write_all(data).unwrap();
        Ok(data.len())
    }
}

pub fn request_authentication(){
    
    let auth_code = "b38c386302478bc002d98e5bc18284c8dd9fa55a";
    let mut post_form = Form::new();

    post_form.part("client_id")
        .contents(CLIENT_ID.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("client_id error"));
    post_form.part("client_secret")
        .contents(CLIENT_SECRET.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("client_secret error"));
    post_form.part("code")
        .contents(auth_code.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("auth code err"));
    post_form.part("grant_type")
        .contents("authorization_code".as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("grant type err"));

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url("https://www.strava.com/oauth/token").unwrap();
    easy2.httppost(post_form).unwrap();
    easy2.perform().unwrap();

    
}

//This will return currently valid short access token and refresh token
pub fn request_new_access_tokens(refresh_token: &RefreshToken) -> Vec<u8> {

    let mut post_form = Form::new();

    post_form.part("client_id")
        .contents(CLIENT_ID.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("client_id error"));
    post_form.part("client_secret")
        .contents(CLIENT_SECRET.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("client_secret error"));
    post_form.part("grant_type")
        .contents("refresh_token".as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("grant type err"));
    post_form.part("refresh_token")
        .contents(refresh_token.token_string.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("refresh token err"));

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url("https://www.strava.com/oauth/token").unwrap();
    easy2.httppost(post_form).unwrap();
    easy2.perform().unwrap();

    let contents = &easy2.get_ref().0;
    contents.to_vec()
}

//TO DO: should request and return contents only. No parsing here.
pub fn request_specific_activity(activity_id: String){
    // let activity_id = "7955070771";
    let url = format!("https://www.strava.com/api/v3/activities/{}?include_all_efforts=false", activity_id);
    let auth_header = format!("Authorization: Bearer {}", get_short_access_token().token_string);

    let mut headers = List::new();
    headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url(url.as_str()).unwrap();
    easy2.get(true).unwrap();
    easy2.http_headers(headers).unwrap();
    easy2.perform().unwrap();

    let contents_string = &easy2.get_ref().0;
    let parsed: Value = serde_json::from_str(str::from_utf8(&contents_string).unwrap()).unwrap();
    let pretty_parsed = format!("{:#}", parsed);
    println!("parsed contents: {}", pretty_parsed);
}

pub fn request_activity_stream(activity_id: String, keys: Vec<String>) -> Vec<u8> {
    // let test_activity_id = "7955070771";
    let auth_code = "Authorization: Bearer ";
    let short_access_token = get_short_access_token();
    let auth_header = format!("{}{}", auth_code, short_access_token.token_string);

    // let keys = vec!["one", "two", "three"];
    let mut url: String = format!("https://www.strava.com/api/v3/activities/{}/streams?", activity_id);
    // "https://www.strava.com/api/v3/activities/{}/streams?keys=time,latlng&key_by_type=true"
    for k in keys{
        url.push_str(&k);
        url.push(',');
    }
    url.push_str("&key_by_type=true");

    println!("{}", url);

    let auth_header = format!("Authorization: Bearer {}", get_short_access_token().token_string);

    let mut headers = List::new();
    headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url(url.as_str()).unwrap_or_else(|err| panic!("problem connecting to activity stream url {:?}", err));
    easy2.get(true).unwrap();
    easy2.http_headers(headers).unwrap();
    
    easy2.perform().unwrap();

    let contents = &easy2.get_ref().0;
    contents.to_vec()
    // let json: Value = serde_json::from_str(str::from_utf8(&contents).unwrap()).unwrap();

    // // println!("distance length = {}", json["distance"].as_array().unwrap().len());
    // // println!("time length = {}", json["time"].as_array().unwrap().len());
    // // println!("latlng length = {}", json["latlng"].as_array().unwrap().len());

    // println!("(DISTANCE) {}", json["distance"]["data"].as_array().unwrap().len());
    // println!("(TIME) {}", json["time"]["data"].as_array().unwrap().len());
    // println!("(LATLNG) {}", json["latlng"]["data"].as_array().unwrap().len());
    // // file.write_all(&contents).unwrap_or_else(|err| {
    // //     panic!("Problem writing GPX data to file: {:?}", err);
    // // });

    // let activity_map = ActivityMap::build_from_json(test_activity_id.to_owned(), json);
}