//-- ##############################
//-- Task: Implementing get request
//-- Author: Tangani Moyo
//-- Version: 1.0.0
//-- Date: 05 April 23
//-- ############################## 197.92.140.60


use std::io;
use reqwest;
use reqwest::{Client, Response};
use regex::Regex;
// use std::string::String;
use std::sync::{Arc, Mutex};
use serde_json::{Result, Value};
// use serde_json::Value::String;
use serde_json::Value::String as OtherString;


pub(crate) fn main() -> &'static str {

    let mut user_option: String = String::new();
    println!("Do you have a unique IP Address to check? (y/N)");
    io::stdin()
        .read_line(&mut user_option)  // .unwrap()
        .expect("failed to read line");

    user_option = user_option.trim().to_ascii_uppercase();
    let option = user_option.as_str();

    let mut ip_address: String = String::new();
    // let _hold;

    match option {
        "Y" => ip_address =  get_known_ip_address(),
        "N" => ip_address =  request_unknown_ip_address(),
        _ => println!("There is an issue here")
    }

    if ip_address != "false" {
        let _location = get_location(ip_address);
    }

    return "This IP Address is from South Africa";
}

fn get_known_ip_address() -> String {
    let mut _ip_address = String::new();
    println!("Enter your ip address :");
    std::io::stdin().read_line(&mut _ip_address).unwrap();

    return _ip_address;
}

#[tokio::main]
async fn request_unknown_ip_address() -> String {

    let client = Client::new();
    let ip_res = client
        .get("http://checkip.dyndns.com/")
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

    let pattern  = Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();

    let hold = ip_res.as_str().clone();

    if let Some(captured) = pattern.captures(hold) {
        let _ip_address = captured.get(0).unwrap().as_str().clone();
        let data = Arc::new(Mutex::new(String::from(_ip_address)));
        let thread_data = Arc::clone(&data);
        let value = thread_data.lock().unwrap();

        return value.as_str().parse().unwrap();

    } else {
        println!("No IP address found.");
        return  "false".parse().unwrap();
    }
}

fn validator(ip_address: &str) -> bool {
    let mut ip_address_parts = ip_address.split(".");
    let first_part: i32 = ip_address_parts.next().unwrap().parse::<i32>().unwrap();
    let second_part: i32 = ip_address_parts.next().unwrap().parse::<i32>().unwrap();
    let third_part: i32 = ip_address_parts.next().unwrap().parse::<i32>().unwrap();
    let mut fourth_part: String = ip_address_parts.next().unwrap().to_string();
    if fourth_part.ends_with("\n") {
        fourth_part.pop();
    } else if fourth_part.ends_with(" ") {
        fourth_part.pop();
    } else if fourth_part.ends_with("\r") {
        fourth_part.pop();
    }
    fourth_part.parse::<i32>().unwrap();

    match first_part {
        0 => println!("Error: 0.x.x.x is reserved for software."),
        10 => println!("Error: 10.x.x.x is reserved for the private network."),
        100 => match second_part {
            64..=127 => println!("Error: 100.64.x.x-100.127.x.x is reserved for the private network."),
            _ => {}
        },
        127 => println!("Error: 127.x.x.x is reserved for the host."),
        172 => match second_part {
            16..=31 => println!("ERROR: 172.16.x.x-172.31.x.x is reserved for the private network."),
            _ => {}
        },
        169 => match second_part {
            254 => println!("ERROR: 169.254.x.x is reserved for the subnet."),
            _ => {}
        },
        192 => match second_part {
           0 => match third_part {
               0 => println!("ERROR: 192.0.0.x is reserved for the private network."),
               2 => println!("ERROR: 192.0.2.x is reserved for documentation."),
               _ => {}
           },
            88 => match third_part {
                99 => println!("ERROR: 192.88.99.x is reserved for 6to4."),
                _ => {}
            }
            168 => println!("ERROR: 192.168.x.x is reserved for the private network."),
            _ => {}
        },
        198 => match second_part {
            18 | 19 => println!("ERROR: 198.18.x.x and 198.19.x.x are reserved for the private network."),
            51 => match third_part {
                100 => println!("ERROR: 198.51.100.x is reserved for documentation."),
                _ => {}
            }
            _ => {}
        },
        203 => match second_part {
            0 => match third_part {
                113 => println!("ERROR: 203.0.113.x is reserved for documentation."),
                _ => {}
            },
            _ => {}
        },
        224..=239 => println!("ERROR: 224.x.x.x-239.x.x.x is reserved for multicast."),
        240..=255 => println!("ERROR: 240.x.x.x-255.x.x.x is reserved."),
        _ => return true
    }
    return false
}

#[tokio::main]
async fn get_location(ip_address: String) -> &'static str {

    println!("Getting location for IP Address: {}", ip_address);

    let client = Client::new();
    let response = client
        .get(format!("http://ip-api.com/json/{}", ip_address))
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");


    // let v: Value = serde_json::from_str(&*location.to_string());
    // let location: ParsedType = serde_json::from_str(&response).unwrap();
    // println!("{:?}", location);

    println!("{:?}", serde_json::from_str::<serde_json::Value>(&*String::from(&response)));

    let _held = serde_json::from_str::<serde_json::Value>(&*String::from(&response));
    println!("{:?}", _held.unwrap());

    // println!("{:?}", _held.unwrap()["country"]);


    // println!("City: {:?}, region: {}, country: {}", v["city"], v["region"], v["country"]);

    // let _location_data = location.json::<Response>().unwrap();
    // println!("{:?}", _location_data);

    println!("{:?}", response);

    return "Stellenbosch";
}