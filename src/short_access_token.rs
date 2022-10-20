use crate::write_to_file;

// use std::io::{stdout, stdin, Read, Write};
// use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
// use serde_json::{Value};
use serde::{Serialize, Deserialize};

use bincode;

// use std::fs::{File, OpenOptions};
// use std::io::prelude::*;
use std::path::Path;
// use std::io::ErrorKind;
use std::str;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortAccessToken{
    pub token_string: String,
    pub exp_date: String,
}

impl ShortAccessToken{
    pub fn build(token_string: String, exp_date: String) -> ShortAccessToken {
        ShortAccessToken { token_string, exp_date }
    }

    pub fn save_short_access_token(&self) {
        let serialized_token = bincode::serialize(&self).unwrap();

        let file_path = Path::new("short_access_token_data.txt");
        let display = file_path.display();
        write_to_file(file_path, serialized_token);
    }

    // pub fn check_if_expired() -> bool {

    // }
    pub fn check_if_expired(&self) -> bool{
        let time_now = Local::now().timestamp();
        let expires = self.exp_date.parse::<i64>().unwrap_or_else(|err| panic!("Can't parse token expiration into i64. ERROR: {:?}", err));
        if time_now > expires {true} else {false}
}

}