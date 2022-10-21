use std::io::{stdout, stdin, Read, Write};

use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use crate::write_to_file;
// use my_running::{RefreshToken, ShortAccessToken};

use bincode;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::str;
// use std::str::pattern::Pattern;
use regex::Regex;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityMap{
    id: String,
    map_points: Vec<ActivityMapPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityMapPoint{
    distance: String,
    time: String,
    latlng: String,
}

impl ActivityMap {
    pub fn build_from_json(id: String, json_data: Value) -> ActivityMap{
        let distance_points = json_data["distance"]["data"].as_array().unwrap();
        let time_points = json_data["time"]["data"].as_array().unwrap_or_else(|| panic!("Error with time points response"));
        let latlng_points = json_data["latlng"]["data"].as_array().unwrap();

        //Check if map points returned from api line up properly
        if (distance_points.len() != time_points.len() || distance_points.len() != latlng_points.len()){
            panic!("distance, time, latlng dont line up");
        }

        let mut map_point_vec: Vec<ActivityMapPoint> = Vec::new();

        for index in 0..distance_points.len(){
            let map_point = ActivityMapPoint{
                distance: distance_points[index].to_string(),
                time: time_points[index].to_string(),
                latlng: latlng_points[index].to_string()
            };

            map_point_vec.push(map_point);
        }

        ActivityMap{id: id, map_points: map_point_vec}
    }

    pub fn save_to_file(&self){
        let file_name = format!("activity_map_{}", self.id);
        let file_path = Path::new(&file_name);
        write_to_file(file_path, bincode::serialize(&self).unwrap());
    }

    pub fn get_activity_map_from_file(id: &str) -> ActivityMap {
        let file_name = format!("activity_map_{}", id);
        let file_path = Path::new(&file_name);

        let mut file = File::open(&file_path).unwrap_or_else(|err| {
            panic!("problem opening file: {:?} Error {:?}", file_path.display(), err);
        });

        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf).unwrap_or_else(|err| {
            panic!("problem reading activity map into buffer {:?}", err);
        });

        let activity_map: ActivityMap = bincode::deserialize(&buf).unwrap_or_else(|err| {
            panic!("problem deserializing map buffer {:?}", err);
        });

        activity_map
    }

    // pub fn get_map_points(&self) -> &Vec<ActivityMapPoint>{
    //     &self.map_points
    // }

    pub fn get_location_points(&self) -> Vec<&String>{
        let mut location_points = Vec::new();
        for mp in &self.map_points{
            location_points.push(&mp.latlng);
        }
        location_points
    }

    pub fn get_time_points(&self) -> Vec<&String>{
        let mut time_points = Vec::new();
        for mp in &self.map_points{
            time_points.push(&mp.time);
        }
        time_points
    }

    pub fn get_distance_points(&self) -> Vec<&String>{
        let mut distance_points = Vec::new();
        for mp in &self.map_points{
            distance_points.push(&mp.distance);
        }
        distance_points
    }

    pub fn show_first_twenty(&self){
        for mp in &self.map_points[0..20]{
            println!("geo: {} - time {} - distance {}", mp.latlng, mp.time, mp.distance);
        }
    }

}