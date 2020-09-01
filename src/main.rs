use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env;

use regex::Regex;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No locations file was specified");
        println!("usage: minecraft_tracker <file_location>");
        return;
    }
    let csv_file_name = &args[1];
    let mut locations: Vec<Location> = load_csv(&csv_file_name);
    loop {
        println!("What would you like to do?");
        println!("[A]dd a place");
        println!("[S]earch a place");
        println!("[Q]uit");
        let cmd = get_input("");
        if cmd == "Q" || cmd == "q" {
            save_csv(&locations, &csv_file_name).expect("Unable to save locations.csv");
            break;
        }
        if cmd == "A" || cmd == "a" {
            add_location(&mut locations);
            save_csv(&locations, &csv_file_name).expect("Unable to save locations.csv");
        }
        if cmd == "S" || cmd == "s" {
            search_location(&locations);
        }
    }
}

const OVERWORLD: i32 = 0;
const NETHER: i32 = 1;
const END: i32 = 2;

struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn add_location(locations: &mut Vec<Location>) {
    let name = get_input("Name");
    let description = get_input("Description");

    let relm = get_input("Relm (O)verworld/(N)ether/(E)nd");
    let relm = match &relm[..] {
        "O" => OVERWORLD,
        "o" => OVERWORLD,
        "N" => NETHER,
        "n" => NETHER,
        "E" => END,
        "e" => END,
        _ => OVERWORLD,
    };

    let coord = Coord {
        x: get_input("X").parse().expect("Invalid number"),
        y: get_input("Y").parse().expect("Invalid number"),
        z: get_input("Z").parse().expect("Invalid number"),
    };

    let loc = Location{
        name: name,
        description: description,
        relm: relm,
        coord: coord,
    };

    locations.push(loc);
}

fn search_location(locations: &Vec<Location>) {
    let query = get_input("Query?").to_lowercase();
    let query = Regex::new(&query[..]).unwrap();
    println!("");
    for loc in locations.iter() {
        match query.captures(&loc.name[..].to_lowercase()) {
            Some(_) => print_location(loc),
            None => continue,
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut line = String::new();

    if prompt != "" {
        println!("{}", prompt);
    }
    std::io::stdin().read_line(&mut line).unwrap();

    return line.trim().to_string();
}

struct Location {
    name: String,
    description: String,
    relm: i32,
    coord: Coord,
}

fn to_csv_line(row: &Location) -> String {
    let ret = row.name.to_owned().replace(",", ".") + "," +
                &row.description.replace(",", ".") + "," +
                &row.relm.to_string() + "," +
                "[ "+&row.coord.x.to_string()+" / "+&row.coord.y.to_string()+" / "+&row.coord.z.to_string()+" ]";
    return ret
}

fn to_location(row: &str) -> Location {
    let fields = row.split(",").collect::<Vec<&str>>();
    let coord_fields = fields[3].split("/").collect::<Vec<&str>>();

    let x = coord_fields[0][1..].trim();
    let x = x.parse().unwrap();
    let y = coord_fields[1].trim();
    let y = y.parse().unwrap();
    let z = coord_fields[2].split("]").collect::<Vec<&str>>()[0].trim();
    let z = z.parse().unwrap();

    let ret = Location{
        name: fields[0].trim().to_string(),
        description: fields[1].trim().to_string(),
        relm: fields[2].parse().unwrap(),
        coord: Coord {
            x: x,
            y: y,
            z: z,
        },
    };

    return ret
}

fn print_location(row: &Location) {
    let relm_name = match row.relm {
        OVERWORLD => "Overworld",
        NETHER => "Nether",
        END => "End",
        _ => "Overworld"
    };

    let name = format!("{}\n------------------------------------", row.name.bold());
    let dets = format!("{}\n------------------------------------", row.description);
    let coord = format!("{}:( {} / {} / {} )", relm_name, row.coord.x, row.coord.y, row.coord.z);

    if row.relm == OVERWORLD {
        println!("{}\n{}\n{}\n{}", 
            name, 
            dets, 
            coord.green(), 
            format!("Nether:( {} / {} / {} )\n", row.coord.x / 8, row.coord.y / 8, row.coord.z / 8).red());
    }
    if row.relm == NETHER {
        println!("{}\n{}\n{}\n{}", 
            name,
            dets,
            coord.red(),
            format!("Overworld:( {} / {} / {} )\n", row.coord.x * 8, row.coord.y * 8, row.coord.z * 8).green());
    }
    if row.relm == END {
        println!("{}\n{}\n{}\n", 
            name,
            dets,
            coord.purple());
    }
}

fn save_csv(locations: &Vec<Location>, file_name: &String) -> std::io::Result<()> {
    let mut data = String::new();
    for loc in locations {
        data += &to_csv_line(loc);
        data += "\n"
    }
    let mut file = File::create(file_name)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn load_csv(file_name: &String) -> Vec<Location> {
    let mut ret: Vec<Location> = Vec::new();
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => return ret,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        ret.push(to_location(&line.unwrap()))
    }

    return ret;
}