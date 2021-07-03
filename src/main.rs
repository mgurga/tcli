use clap::{Arg, App};
extern crate json;

fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("My Super Program")
        .version("0.1")
        .about("control tauon music box from the command line")
        .arg(Arg::new("url")
            .about("api url")
            .takes_value(true)
            .default_value("http://localhost:7814/api1"))
        .arg("--play-pause 'Switches play to pause and vice versa'")
        .get_matches();

    if matches.is_present("play-pause") {
        println!("getting: {:?}", format!("{}/status", matches.value_of("url").unwrap()));
        let resp = reqwest::blocking::Client::new()
            .get(format!("{}/status", matches.value_of("url").unwrap()))
            .send()?
            .text();
        println!("{:#?}", &resp.as_ref().unwrap());
        let respjson = json::parse(&resp.unwrap()).unwrap();
        println!("player status: {}", respjson["status"].as_str().unwrap());
        match respjson["status"].as_str().unwrap() {
            "paused" | "stopped" => {
                println!("paused, going to play");
                let playresp = reqwest::blocking::Client::new()
                    .get(format!("{}/play", matches.value_of("url").unwrap()))
                    .send()?
                    .text();
                if playresp.unwrap() == "OK" {
                    println!("playing...");
                }
            }
            "playing" => {
                println!("playing, going to pause");
                let playresp = reqwest::blocking::Client::new()
                    .get(format!("{}/pause", matches.value_of("url").unwrap()))
                    .send()?
                    .text();
                if playresp.unwrap() == "OK" {
                    println!("pausing...");
                }
            }
            _ => {
                println!("what was sent???");
            }
        }
    }

    Ok(())
}
