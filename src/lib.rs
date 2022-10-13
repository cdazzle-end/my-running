
use std::io::{stdout, stdin, Read, Write};
use curl::easy::{Easy, Easy2, List, Form, Part, Handler, WriteError};
use serde_json::{Value};
use serde::{Serialize, Deserialize};

use bincode;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;

//Byte buffer that will be wrapped in the Easy handler and collect data recieved from HTTP requests
pub struct Collector(pub Vec<u8>);

//Required trait to implement when creating an Easy handler, writes data into the byte buffer
impl Handler for Collector{
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }
}

//RefreshToken used when short access token expires, we send refresh token to strava and get a new short access token.
#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshToken{
    pub token_string: String,
    pub exp_date: String,
}

impl RefreshToken{
    pub fn build(token_string: String, exp_date: String) -> RefreshToken {
        RefreshToken { token_string, exp_date }
    }

    //Serialize refresh token and save it to a local file
    pub fn save_refresh_token(&self){
        let serialized_token = bincode::serialize(&self).unwrap();
        let file_path = Path::new("refresh_token_data.txt");
        let display = file_path.display();
        write_to_file(file_path, serialized_token)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortAccessToken{
    token_string: String,
    exp_date: String,
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
        // let mut file = OpenOptions::new().write(true).open(&file_path).unwrap_or_else(|err| {
        //     if err.kind() == ErrorKind::NotFound {
        //         File::create(&file_path).unwrap_or_else(|err| {
        //             panic!("Problem creating file: {:?}", err);
        //         })
        //     } else {
        //         panic!("Problem opening file: {:?}", err);
        //     }
        // });

        // file.write_all(&serialized_token).unwrap_or_else(|err| {
        //     panic!("Problem writing stoken to file: {:?}", err);
        // });

        // println!("success writing stoken to file {:?}", display);
    }

}

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

    deserialized_token
}