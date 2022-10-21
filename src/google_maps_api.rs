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

use crate::activity_map::ActivityMap;
use crate::{get_last_refresh_token, write_to_file, activity_map};
use crate::get_short_access_token;
use crate::refresh_token::RefreshToken;
use crate::short_access_token::ShortAccessToken;
use crate::read_json;
use crate::remove_extra_characters;

use crate::strava_api::Collector;

// https://maps.googleapis.com/maps/api/staticmap?parameters
pub fn make_maps_request(activity_map: ActivityMap){
    //required parameters
    let location_points = activity_map.get_location_points();
    let zoom = "15";
    let size = "&size=500x400";
    let key = "&key=AIzaSyCWCjvDCKLG7jaSvs1vngECd80HM-LAdIM";
    // let signature = ""
    let mut path_string: String = "path=".to_string();
    let mut condensed_location_points = condense_location_points(location_points);
    for lp in condensed_location_points{
        // println!("{}", lp.replace(&['[',']'][..], ""));
        let clean_location_point = lp.replace(&['[',']'][..], "");
        path_string.push_str(&clean_location_point);
        path_string.push('|');
    }
    path_string.pop();
    let url_base = "https://maps.googleapis.com/maps/api/staticmap?";
    let complete_url = format!("{}{}{}{}", url_base, path_string, size, key);
    println!("{}", path_string);
    // let url = "https://maps.googleapis.com/maps/api/staticmap?center=33.667686,-78.947315&zoom=15&size=400x400&key=AIzaSyCWCjvDCKLG7jaSvs1vngECd80HM-LAdIM";

    let mut easy2 = Easy2::new(Collector(Vec::new()));
    easy2.url(complete_url.as_str()).unwrap();
    easy2.get(true).unwrap();
    // easy2.http_headers(headers).unwrap();
    easy2.perform().unwrap();

    let contents = &easy2.get_ref().0;
    // println!("{:?}", contents);

    let map_file = Path::new("map_images/test_map_2.png");
    write_to_file(map_file, contents.to_vec());
}

fn condense_location_points(location_points: Vec<&String>) -> Vec<String> {
    let max_url_chars = 8000;
    let most_locations = max_url_chars/19;

    //remove 75%
    let mut index = 0;
    let mut condensed_location_points: Vec<String> = Vec::new();
    for lp in location_points{
        if index % 4 == 0 {
            condensed_location_points.push(lp.to_string());
        }
        index += 1;
    }
    condensed_location_points
}