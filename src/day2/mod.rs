use std::fs::File;
use std::io::{ BufReader, BufRead };

enum Movement
{
    Forward( i32 ),
    Up( i32 ),
    Down( i32 )
}

fn get_input
    (
    path : &str
    ) -> Vec<Movement>
{
    let mut v : Vec<Movement> = Vec::new();
    let file = match File::open( path )
    {
        Err( why ) => panic!( "Couldn't open {}: {}", path, why ),
        Ok( file ) => file
    };
    for line in BufReader::new( file ).lines()
    {
        v.push( match line
        {
            Ok( s ) =>
            {
                let pair_vec = s.split( " " ).collect::<Vec<&str>>();
                let dist =  match pair_vec[1].parse::<i32>()
                {
                    Ok( n ) => n,
                    Err( e ) => panic!( "Unable to parse distance: {}", e )
                };
                match pair_vec[0]
                {
                    "forward" => Movement::Forward( dist ),
                    "up" => Movement::Up( dist ),
                    "down" => Movement::Down( dist ),
                    _ => panic!( "Unknown direction" )
                }
            }
            Err( e ) => panic!( "Unable to read line: {}", e )
        } );
    }
    return v;
}

pub fn part1() -> String
{
    let mut horizontal : i32 = 0;
    let mut depth : i32 = 0;
    let movement = get_input( "../../res/day2/input.txt" );
    for m in movement
    {
        match m
        {
            Movement::Forward( d ) => horizontal += d,
            Movement::Up( d ) => depth -= d,
            Movement::Down( d ) => depth += d
        }
    }
    return ( horizontal * depth ).to_string();
}

pub fn part2() -> String
{
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let movement = get_input( "../../res/day2/input.txt" );
    for m in movement
    {
        match m
        {
            Movement::Forward( d ) =>
            {
                horizontal += d;
                depth += aim * d;
            },
            Movement::Up( d ) =>
            {
                aim -= d;
            },
            Movement::Down( d ) =>
            {
                aim += d;
            }
        }
    }
    return ( horizontal * depth ).to_string();
}
