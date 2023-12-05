use std::fs;

fn main() {
    let games = fs::read_to_string("./input").expect("Could not read file!");
    let mut pt1 = 0;
    let mut pt2 = 0;
    for game in games.lines() {
        let mut game_valid = true;
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        let gamestr = game.chars().filter(|ch| !ch.is_whitespace()).collect::<String>();
        let game_nr = gamestr.split(":").next().expect("ERROR")
        .chars().filter(|ch| ch.is_ascii_digit()).collect::<String>();
        let game_shows = gamestr.split(":").skip(1).next().expect("ERROR").split(";");

        for show in game_shows {
            let show_instance = show.split(",");
            for instance in show_instance {
                let instance_nr = instance.chars().filter(|ch| ch.is_ascii_digit()).collect::<String>().parse::<u8>().expect("ERROR");
                let instance_color = instance.chars().filter(|ch| !ch.is_ascii_digit()).collect::<String>();
                if instance_color == "red" && instance_nr > 12 {game_valid = false};
                if instance_color == "green" && instance_nr > 13 {game_valid = false};
                if instance_color == "blue" && instance_nr > 14 {game_valid = false};
                
                if instance_color == "red" && instance_nr > red_max {red_max = instance_nr};
                if instance_color == "green" && instance_nr > green_max {green_max = instance_nr};
                if instance_color == "blue" && instance_nr > blue_max {blue_max = instance_nr};
                
            }
        }
        
        pt2 += red_max as u32 * green_max as u32 * blue_max as u32;
        if game_valid {
            pt1 += game_nr.parse::<u32>().expect("ERROR");
        }
    }

    println!("pt1 = {}", pt1);
    println!("pt2 = {}", pt2);
}
