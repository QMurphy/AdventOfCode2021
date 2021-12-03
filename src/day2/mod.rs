use std::fs::File;
use std::io::{ BufReader, BufRead };

/// A Movement enum with three variants that hold integers
enum Movement
{
    Forward( i32 ),
    Up( i32 ),
    Down( i32 )
}

/// Parses an input file into a Vec of Movements
fn get_input
    (
    path : &str
    ) -> Vec<Movement>
{
    // Create return vector
    let mut v : Vec<Movement> = Vec::new();
    
    // Open the input, panic otherwise
    let file = match File::open( path )
    {
        Err( why ) => panic!( "Couldn't open {}: {}", path, why ),
        Ok( file ) => file
    };
    
    // For each line
    for line in BufReader::new( file ).lines()
    {
        // Push the Movement from the line into v, panic otherwise
        v.push( match line
        {
            Ok( s ) =>
            {
                // Split the string by spaces
                let pair_vec = s.split( " " ).collect::<Vec<&str>>();

                // Try to parse the second string to an i32, panic otherwise
                let dist =  match pair_vec[1].parse::<i32>()
                {
                    Ok( n ) => n,
                    Err( e ) => panic!( "Unable to parse distance: {}", e )
                };

                // Try to parse the first string to an enum variant, panic otherwise
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

    // For each movement
    for m in movement
    {
        // Apply changes to variables
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
    
    // For each movement
    for m in movement
    {
        // Apply changes to variables
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
