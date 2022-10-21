use std::io::{stdout, stdin, Read, Write};

use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use my_running::*;
// use my_running::{RefreshToken, ShortAccessToken};

use bincode;
// use 
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::str;
// use std::str::pattern::Pattern;
use regex::Regex;
use chrono::prelude::*;

// https://maps.googleapis.com/maps/api/staticmap?center=Berkeley,CA&zoom=14&size=400x400&key=AIzaSyCWCjvDCKLG7jaSvs1vngECd80HM-LAdIM

// http://www.strava.com/oauth/authorize?client_id=94993&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=read_all,profile:read_all,activity:read_all
// http://localhost/exchange_token?state=&code=b38c386302478bc002d98e5bc18284c8dd9fa55a&scope=read
fn main() {

    // get_short_access_token();
    // get_sp();
    // get_activity_stream();
    // load_activity_map();
    get_google_maps();
   

}

// fn get_athlete_activities_list(){
//     let auth_code = "Authorization: Bearer ";
//     let short_access_token = get_short_access_token();
//     let auth_header = format!("{}{}", auth_code, short_access_token.token_string);
//     println!("auth_header: {}", auth_header);


//     let mut headers = List::new();
//     headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

//     let mut easy2 = Easy2::new(Collector(Vec::new()));
//     easy2.url("https://www.strava.com/api/v3/athlete/activities?before=1665799680&after=&page=1&per_page=30").unwrap();
//     easy2.get(true).unwrap();
//     easy2.http_headers(headers).unwrap();
//     easy2.perform().unwrap();
// }

// //Get activity by ID and include_all_efforts
// fn get_specific_activity(){
//     let test_activity_id = "7955070771";
//     let auth_code = "Authorization: Bearer ";
//     let short_access_token = get_short_access_token();
//     let auth_header = format!("{}{}", auth_code, short_access_token.token_string);
//     println!("auth_header: {}", auth_header);

//     let mut headers = List::new();
//     headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

//     let mut easy2 = Easy2::new(Collector(Vec::new()));
//     easy2.url("https://www.strava.com/api/v3/activities/7955070771?include_all_efforts=false").unwrap();
//     easy2.get(true).unwrap();
//     easy2.http_headers(headers).unwrap();
//     easy2.perform().unwrap();

//     println!("");
//     println!("contents: {:?}", easy2.get_ref().0);
//     let contents_string = &easy2.get_ref().0;
//     let parsed: Value = serde_json::from_str(str::from_utf8(&contents_string).unwrap()).unwrap();
//     // parsed.as_object()
//     let pretty_parsed = format!("{:#}", parsed);
//     println!("parsed contents: {}", pretty_parsed);
// }

// fn download_gpx_file(){
//     let test_activity_id = "7955070771";
//     let auth_code = "Authorization: Bearer ";
//     let short_access_token = get_short_access_token();
//     let auth_header = format!("{}{}", auth_code, short_access_token.token_string);

//     let mut headers = List::new();
//     headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

//     let mut easy2 = Easy2::new(Collector(Vec::new()));
//     easy2.url("https://www.strava.com/api/v3/routes/7955070771/export_gpx").unwrap_or_else(|err| panic!("problem connecting to export_gpx url. {:?}", err));
//     easy2.get(true).unwrap();
//     easy2.http_headers(headers).unwrap();

//     let file_path = Path::new("gpx_file.gpx");


//     let mut file = OpenOptions::new().write(true).open(file_path).unwrap_or_else(|err| {
//         if err.kind() == ErrorKind::NotFound {
//             File::create(file_path).unwrap_or_else(|err| {
//                 panic!("Problem creating file: {:?}", err);
//             })
//         } else {
//             panic!("Problem opening file: {:?}", err);
//         }
//     });

    
//     easy2.perform().unwrap();

//     let contents = &easy2.get_ref().0;
//     file.write_all(&contents).unwrap_or_else(|err| {
//         panic!("Problem writing GPX data to file: {:?}", err);
//     });
// }


pub fn epoch_to_est_time(time: String){

}


// fn create_activity_map_from_json(){

// }








// fn build_short_access_token(token_string: String, exp_date: String) -> ShortAccessToken {
//     ShortAccessToken { token_string, exp_date }
// }
// {"token_type":"Bearer","access_token":"f3dd7c70b07bd2e6861fb5cb3e280d16c19c9187","expires_at":1665538974,"expires_in":21600,"refresh_token":"2f4bf8fb46aa125f1d26edf97bb8cdb4016d3348"}