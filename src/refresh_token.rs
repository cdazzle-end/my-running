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
// use chrono::prelude::*;

//RefreshToken used when short access token expires, we send refresh token to strava and get a new short access token.
#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshToken{
    pub token_string: String,
}

impl RefreshToken{
    pub fn build(token_string: String) -> RefreshToken {
        RefreshToken {token_string}
    }

    //Serialize refresh token and save it to a local file
    pub fn save_refresh_token(&self){
        let serialized_token = bincode::serialize(&self).unwrap();
        let file_path = Path::new("refresh_token_data.txt");
        let display = file_path.display();
        write_to_file(file_path, serialized_token)
    }
}