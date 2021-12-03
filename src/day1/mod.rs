use std::fs::File;
use std::io::{ BufReader, BufRead };

/// Parses an input file into a Vec of i32s
fn get_input
    (
    path : &str
    ) -> Vec<i32>
{
    // Create return vector
    let mut v : Vec<i32> = Vec::new();

    // Open the input, panic otherwise
    let file = match File::open( path )
    {
        Err( why ) => panic!( "Couldn't open {}: {}", path, why ),
        Ok( file ) => file
    };

    // For each line
    for line in BufReader::new( file ).lines()
    {
        // Push the i32 from the line into v, panic otherwise
        v.push( match line
        {
            // Try to parse the string to an i32, panic otherwise
            Ok( s ) => match s.parse::<i32>()
            {
                Ok( n ) => n,
                Err( e ) => panic!( "Unable to parse number from line: {}", e )
            }
            Err( e ) => panic!( "Unable to read line: {}", e )
        } );
    }
    return v;
}

pub fn part1() -> String
{
    let mut num_increasing = 0;
    let depths = get_input( "../../res/day1/input.txt" );

    // For each depth reading
    for i in 1..depths.len()
    {
        // Increment num_increasing if it increased
        if( depths[i] > depths[i - 1] )
        {
            num_increasing += 1;
        }
    }
    return num_increasing.to_string();
}

pub fn part2() -> String
{
    let mut num_increasing = 0;
    let depths = get_input( "../../res/day1/input.txt" );

    // For each depth reading
    for i in 3..depths.len()
    {
        // Increment num_increasing if it increased with a moving window of 3
        if( depths[i] + depths[i - 1] + depths[i - 2] > depths[i - 1] + depths[i - 2] + depths[i - 3] )
        {
            num_increasing += 1;
        }
    }
    return num_increasing.to_string();
}
