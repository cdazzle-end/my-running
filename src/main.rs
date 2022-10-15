use std::io::{stdout, stdin, Read, Write};

use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use my_running::*;
// use my_running::{RefreshToken, ShortAccessToken};

use bincode;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::str;
// use std::str::pattern::Pattern;
use regex::Regex;

// http://www.strava.com/oauth/authorize?client_id=94993&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=read_all,profile:read_all,activity:read_all
// http://localhost/exchange_token?state=&code=b38c386302478bc002d98e5bc18284c8dd9fa55a&scope=read
fn main() {

    // let auth_code = "Authorization: Bearer 4753502b2b1473c576c20d2667272233ad076eec ";
    // get_authenticated();
    // handle_json();
    // save_refresh_token();
    // get_refresh_token();
    // refresh_access_token();
    // let sat = get_short_access_token();
    // println!("sat string {}, exp {}", sat.token_string, sat.exp_date);
    // let rt = get_last_refresh_token();
    // rt.save_refresh_token()
    // println!("rt string {}, exp {}", rt.token_string, rt.exp_date);
    // remove_extra_characters();
    // get_short_access_token();
    // get_athlete_activities_list();
    get_specific_activity();

    
    // let contents = serde_json::json!({"achievement_count":0,"athlete":{"id":88850079,"resource_state":1},"athlete_count":1,"available_zones":[],"average_speed":2.625,"best_efforts":[{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":400,"elapsed_time":144,"end_index":971,"id":21979946231,"moving_time":145,"name":"400m","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:38:57Z","start_date_local":"2022-10-12T22:38:57Z","start_index":859},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":805,"elapsed_time":296,"end_index":1388,"id":21979946236,"moving_time":297,"name":"1/2 mile","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:45:07Z","start_date_local":"2022-10-12T22:45:07Z","start_index":1152},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":1000,"elapsed_time":370,"end_index":1381,"id":21979946237,"moving_time":371,"name":"1k","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:43:43Z","start_date_local":"2022-10-12T22:43:43Z","start_index":1086},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":1609,"elapsed_time":597,"end_index":1398,"id":21979946239,"moving_time":598,"name":"1 mile","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:40:17Z","start_date_local":"2022-10-12T22:40:17Z","start_index":923},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":3219,"elapsed_time":1219,"end_index":1397,"id":21979946241,"moving_time":1220,"name":"2 mile","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:29:54Z","start_date_local":"2022-10-12T22:29:54Z","start_index":426}]});
    // let contents: Value = serde_json::from_str(r#"{"achievement_count":0,"athlete":{"id":88850079,"resource_state":1},"athlete_count":1,"available_zones":[],"average_speed":2.625,"best_efforts":[{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":400,"elapsed_time":144,"end_index":971,"id":21979946231,"moving_time":145,"name":"400m","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:38:57Z","start_date_local":"2022-10-12T22:38:57Z","start_index":859},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":805,"elapsed_time":296,"end_index":1388,"id":21979946236,"moving_time":297,"name":"1/2 mile","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:45:07Z","start_date_local":"2022-10-12T22:45:07Z","start_index":1152},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":1000,"elapsed_time":370,"end_index":1381,"id":21979946237,"moving_time":371,"name":"1k","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:43:43Z","start_date_local":"2022-10-12T22:43:43Z","start_index":1086},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":1609,"elapsed_time":597,"end_index":1398,"id":21979946239,"moving_time":598,"name":"1 mile","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:40:17Z","start_date_local":"2022-10-12T22:40:17Z","start_index":923},{"achievements":[],"activity":{"id":7955070771,"resource_state":1},"athlete":{"id":88850079,"resource_state":1},"distance":3219,"elapsed_time":1219,"end_index":1397,"id":21979946241,"moving_time":1220,"name":"2 mile","pr_rank":null,"resource_state":2,"start_date":"2022-10-13T02:29:54Z","start_date_local":"2022-10-12T22:29:54Z","start_index":426}]}"#).unwrap();

    // let pretty = format!("{:#}", contents);
    // println!("{}", pretty);


}

fn test_easy(auth_code: &str){
    let mut list = List::new();
    list.append(auth_code).unwrap();
    let mut handle = Easy::new();
    // handle.url("https://www.strava.com/api/v3/athlete/activities").unwrap();
    handle.url("https://www.strava.com/api/v3/athlete").unwrap();
    handle.get(true).unwrap();
    handle.http_headers(list).unwrap();

    handle.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    
    handle.perform().unwrap();
}

//Easy2 implements its methods via the Handler trait.
//Create a struct that impl's the Handler trait
//Then create a new Easy2 using this struct. I guess Easy2 serves like a wrapper
fn test_easy2(auth_code: &str){
    


    let mut headers = List::new();
    headers.append(auth_code).unwrap();

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    // easy2.url("https://www.strava.com/oauth/token").unwrap();
    easy2.url("https://www.strava.com/api/v3/athlete").unwrap();
    easy2.get(true).unwrap();
    easy2.http_headers(headers).unwrap();
    easy2.perform().unwrap();
}

fn get_athlete_activities_list(){
    let auth_code = "Authorization: Bearer ";
    let short_access_token = get_short_access_token();
    let auth_header = format!("{}{}", auth_code, short_access_token.token_string);
    println!("auth_header: {}", auth_header);


    let mut headers = List::new();
    headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url("https://www.strava.com/api/v3/athlete/activities?before=1665799680&after=&page=1&per_page=30").unwrap();
    easy2.get(true).unwrap();
    easy2.http_headers(headers).unwrap();
    easy2.perform().unwrap();
}

//Get activity by ID and include_all_efforts
fn get_specific_activity(){
    let test_activity_id = "7955070771";
    let auth_code = "Authorization: Bearer ";
    let short_access_token = get_short_access_token();
    let auth_header = format!("{}{}", auth_code, short_access_token.token_string);
    println!("auth_header: {}", auth_header);

    let mut headers = List::new();
    headers.append(&auth_header).unwrap_or_else(|err| panic!("failed to add header. Err: {:?}", err));

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url("https://www.strava.com/api/v3/activities/7955070771?include_all_efforts=false").unwrap();
    easy2.get(true).unwrap();
    easy2.http_headers(headers).unwrap();
    easy2.perform().unwrap();

    println!("");
    println!("contents: {:?}", easy2.get_ref().0);
    let contents_string = &easy2.get_ref().0;
    let parsed: Value = serde_json::from_str(str::from_utf8(&contents_string).unwrap()).unwrap();
    // parsed.as_object()
    let pretty_parsed = format!("{:#}", parsed);
    println!("parsed contents: {}", pretty_parsed);
}

fn get_authenticated(){

    let client_id = "94993";
    let client_secret = "a5ce4ce75a78b46db119559a85e12833e390b8f6";
    let auth_code = "b38c386302478bc002d98e5bc18284c8dd9fa55a";

    let mut post_form = Form::new();

    post_form.part("client_id")
        .contents(client_id.as_bytes())
        .add()
        .unwrap_or_else(|err| panic!("client_id error"));
    post_form.part("client_secret")
        .contents(client_secret.as_bytes())
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

// fn handle_json(){
//     let json_string = r#"{"token_type":"Bearer","expires_at":1665396221,"expires_in":21600,"refresh_token":"2f4bf8fb46aa125f1d26edf97bb8cdb4016d3348","access_token":"17406a886120ff41ecd90e2cc0c1912c098a14a4","athlete":{"id":88850079,"username":null,"resource_state":2,"firstname":"Chazzle","lastname":"Dazzle","bio":null,"city":null,"state":null,"country":null,"sex":"M","premium":false,"summit":false,"created_at":"2021-07-14T04:52:04Z","updated_at":"2021-12-24T02:40:47Z","badge_type_id":0,"weight":0.0,"profile_medium":"https://lh3.googleusercontent.com/a/ALm5wu0at48yPsFLQvz3cV1TtN6wqaWcFbwWQG5QFDNt=s96-c","profile":"https://lh3.googleusercontent.com/a/ALm5wu0at48yPsFLQvz3cV1TtN6wqaWcFbwWQG5QFDNt=s96-c","friend":null,"follower":null}}"#;

//     let parsed: Value = read_json(json_string);

//     // println!("Athlete: {} {}", parsed["athlete"]["firstname"], parsed["athlete"]["lastname"]);
//     // println!("Access token: {}", parsed["access_token"])

// }









// fn build_short_access_token(token_string: String, exp_date: String) -> ShortAccessToken {
//     ShortAccessToken { token_string, exp_date }
// }
// {"token_type":"Bearer","access_token":"f3dd7c70b07bd2e6861fb5cb3e280d16c19c9187","expires_at":1665538974,"expires_in":21600,"refresh_token":"2f4bf8fb46aa125f1d26edf97bb8cdb4016d3348"}