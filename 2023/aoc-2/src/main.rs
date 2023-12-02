use std::str::FromStr;
use strum_macros::EnumString;
use json::JsonValue;
use std::fs;
use std::collections::HashMap;
use std::cmp::Eq;

#[derive(Debug,EnumString,Clone,Hash,PartialEq,Eq)]
enum Color {
    #[strum(serialize = "blue")]
    Blue,
    #[strum(serialize = "green")]
    Green,
    #[strum(serialize = "red")]
    Red
}

struct ColorCount {
    color : Color,
    count : u32
}

fn main() {
    let contents = fs::read_to_string("input-aoc-23-02.txt.json").unwrap();
    //let contents = fs::read_to_string("sample.txt.json").unwrap();
    let jsondoc = json::parse(contents.as_str()).unwrap();
    let c = Color::from_str("red").unwrap();

    let limits : HashMap<Color, u32> =
        [ ( Color::Red, 12 ),
          ( Color::Green, 13 ),
          ( Color::Blue, 14 ) ]
          .iter().cloned().collect();  
            
    let mut game_id_sum = 0;
    let mut game_prod = 0;
    if let JsonValue::Array(v) = jsondoc {
        for jsongame in v {
            let game_id : u32 = jsongame["id"].as_u32().unwrap();
            let mut max_green : u32 = 0;
            let mut max_red : u32 = 0;
            let mut max_blue : u32 = 0;
            if let JsonValue::Array(plays) = &jsongame["plays"] {
                let mut game_ok : bool = true;
                for pair in plays {
                    let check_color = Color::from_str(pair[1].as_str().unwrap()).unwrap();
                    let limit_val = limits[&check_color];
                    let check_val = pair[0].as_u32().unwrap();
                    if check_val > limit_val {
                        game_ok = false;
                    }

                    match check_color {
                        Color::Red => max_red = std::cmp::max(max_red, check_val),
                        Color::Green => max_green = std::cmp::max(max_green, check_val),
                        Color::Blue => max_blue = std::cmp::max(max_blue, check_val)
                    }
                }
                if game_ok {
                    game_id_sum = game_id_sum + game_id;
                }
                game_prod = game_prod + dbg!( max_red * max_green * max_blue )
            } else {
                panic!("At the drugstore");
            }
            
        }
    } else {
        panic!("At the disco");
    }
    println!("Got part 1: {}", game_id_sum);
    println!("Got part 2: {}", game_prod);
}
