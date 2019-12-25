mod state_query;
mod access_point_search;

use std::io;

fn main() {

    let mut state_input = String::new();

    println!("Enter state abbreivation for quicker search:");
    match io::stdin().read_line(&mut state_input) {
        Ok(_) => {
            state_input = state_input.trim().to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    let value = state_query::state_boundaries(state_input.to_ascii_uppercase());

    if value == [92.0,92.0,181.0,181.0]{
        println!("Failed to find state");
        return;
    }
    println!("Located state coordinates");

    let mut bssid_input = String::new();

    println!("Enter the BSSID to search");
    println!("Case sensitive!");
    match io::stdin().read_line(&mut bssid_input) {
        Ok(_) => {
            bssid_input = bssid_input.trim().to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let mut api_key = String::new();
    println!("WiGLE API Key (Basic [API]):");
    match io::stdin().read_line(&mut api_key) {
        Ok(_) => {
            api_key = api_key.trim().to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let (target_latitude, target_longtitude) = access_point_search::search(value[0],value[1],value[2],value[3],bssid_input,api_key).unwrap();
    println!("Lat: {}", target_latitude);
    println!("Long: {}", target_longtitude);
}
