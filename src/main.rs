use clap::{App, Arg};
extern crate json;

fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("tcli")
        .version("0.1")
        .about("control tauon music box from the command line")
        .arg(
            Arg::new("url")
                .about("api url")
                .takes_value(true)
                .default_value("http://localhost:7814/api1"),
        )
        .arg("--play-pause 'Switches play to pause and vice versa'")
        .arg("--play 'Start playback'")
        .arg("--stop 'Stop playback'")
        .arg("--pause 'Pause playback'")
        .arg("--next 'Next song in playlist'")
        .arg("--prev 'Previous song in playlist'")
        .arg(
            Arg::new("seek")
                .about("Seeks x milliseconds ahead or behind")
                .takes_value(true)
                .long("seek")
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::new("changevolume")
                .about("Change volume by x units, anything in between -100 and 100")
                .takes_value(true)
                .long("change-volume")
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::new("setvolume")
                .about("Set volume to x units, anything in between 0 and 100")
                .takes_value(true)
                .long("set-volume")
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::new("status")
                .about("Prints current player status in customizable format
    %A = Album
    %T = Track Name
    %N = Track Number
    %R = Artist
    %P = Song Progress (in seconds)
    %D = Song Duration (in seconds)\n")
                .takes_value(true)
                .long("status")
                .default_value("%R - %A - %T")
                .allow_hyphen_values(true),
        )
        .get_matches();

    if matches.is_present("play-pause") {
        println!(
            "getting: {:?}",
            format!("{}/status", matches.value_of("url").unwrap())
        );
        let resp = reqwest::blocking::Client::new()
            .get(format!("{}/status", matches.value_of("url").unwrap()))
            .send()?
            .text();
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
                    std::process::exit(0);
                }
            }
            "playing" => {
                println!("playing, going to pause");
                let pauseresp = reqwest::blocking::Client::new()
                    .get(format!("{}/pause", matches.value_of("url").unwrap()))
                    .send()?
                    .text();
                if pauseresp.unwrap() == "OK" {
                    println!("pausing...");
                    std::process::exit(0);
                }
            }
            _ => {
                println!("what was sent???");
            }
        }
    }

    if matches.is_present("play") {
        let playresp = reqwest::blocking::Client::new()
            .get(format!("{}/play", matches.value_of("url").unwrap()))
            .send()?
            .text();
        if playresp.unwrap() == "OK" {
            println!("playing...");
            std::process::exit(0);
        }
    }

    if matches.is_present("stop") || matches.is_present("pause") {
        let pauseresp = reqwest::blocking::Client::new()
            .get(format!("{}/pause", matches.value_of("url").unwrap()))
            .send()?
            .text();
        if pauseresp.unwrap() == "OK" {
            println!("pausing...");
            std::process::exit(0);
        }
    }

    if matches.is_present("next") {
        let nextresp = reqwest::blocking::Client::new()
            .get(format!("{}/next", matches.value_of("url").unwrap()))
            .send()?
            .text();
        if nextresp.unwrap() == "OK" {
            println!("going to next song...");
            std::process::exit(0);
        }
    }

    if matches.is_present("prev") {
        let prevresp = reqwest::blocking::Client::new()
            .get(format!("{}/back", matches.value_of("url").unwrap()))
            .send()?
            .text();
        if prevresp.unwrap() == "OK" {
            println!("going to previous song...");
            std::process::exit(0);
        }
    }

    if matches.is_present("seek") {
        let resp = reqwest::blocking::Client::new()
            .get(format!("{}/status", matches.value_of("url").unwrap()))
            .send()?
            .text();
        let respjson = json::parse(&resp.unwrap()).unwrap();
        let mut newprogress: i32 = respjson["progress"].as_i32().unwrap()
            + matches.value_of("seek").unwrap().parse::<i32>().unwrap();
        newprogress = std::cmp::max(0, newprogress);
        let seekresp = reqwest::blocking::Client::new()
            .get(format!(
                "{}/seek/{}",
                matches.value_of("url").unwrap(),
                newprogress
            ))
            .send()?
            .text();
        if seekresp.unwrap() == "OK" {
            println!("seeking...");
            std::process::exit(0);
        }
    }

    if matches.is_present("changevolume") {
        let prevresp = reqwest::blocking::Client::new()
            .get(format!(
                "{}/setvolumerel/{}",
                matches.value_of("url").unwrap(),
                matches.value_of("changevolume").unwrap().parse::<i32>().unwrap()
            ))
            .send()?
            .text();
        if prevresp.unwrap() == "OK" {
            println!("changing volume...");
            std::process::exit(0);
        }
    }

    if matches.is_present("setvolume") {
        let prevresp = reqwest::blocking::Client::new()
            .get(format!(
                "{}/setvolume/{}",
                matches.value_of("url").unwrap(),
                matches.value_of("setvolume").unwrap().parse::<i32>().unwrap()
            ))
            .send()?
            .text();
        if prevresp.unwrap() == "OK" {
            println!("setting volume...");
            std::process::exit(0);
        }
    }

    if matches.is_present("status") {
        let resp = reqwest::blocking::Client::new()
            .get(format!("{}/status", matches.value_of("url").unwrap()))
            .send()?
            .text();
        let respjson = json::parse(&resp.unwrap()).unwrap();
        let mut out = String::from(matches.value_of("status").unwrap());
        out = out.replace("%A", respjson["album"].as_str().unwrap_or(""));
        out = out.replace("%T", respjson["title"].as_str().unwrap_or(""));
        out = out.replace("%N", respjson["track"]["track_number"].as_str().unwrap_or(""));
        out = out.replace("%R", respjson["artist"].as_str().unwrap_or(""));
        out = out.replace("%P", &respjson["progress"].as_u32().unwrap_or(0).to_string());
        out = out.replace("%D", &respjson["track"]["duration"].as_u32().unwrap_or(0).to_string());
        println!("{}", out);
        std::process::exit(0);
    }

    println!("something failed");
    Ok(())
}
