//-- ##############################
//-- Task: Implementing get request
//-- Author: Tangani Moyo
//-- Version: 1.0.0
//-- Date: 05 April 23
//-- ############################## 197.92.140.60

use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}


pub(crate) fn main() -> &'static str {
    let mut ip_address = String::new();
    println!("Enter your ip address :");
    std::io::stdin().read_line(&mut ip_address).unwrap();

    let ip_address_validity: bool = validator(&ip_address);
    if ip_address_validity {
        let results = get_location(&ip_address);
    }

    return "This IP Address is from South Africa";
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

async fn get_location(_ip_address: &str) -> Result<(), Box<dyn std::error::Error>>{
    println!("Getting the data");

    let client = reqwest::Client::new();
    let resp200 = client.get("https://httpbin.org/ip")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<GETAPIResponse>()
        .await?;

    println!("{:#?}", resp200);
    println!("Done..");

    Ok(())
}