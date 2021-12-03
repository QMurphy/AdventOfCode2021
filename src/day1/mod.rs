use std::fs::File;
use std::io::{ BufReader, BufRead };

fn get_input
    (
    path : &str
    ) -> Vec<i32>
{
    let mut v : Vec<i32> = Vec::new();
    let file = match File::open( path )
    {
        Err( why ) => panic!( "Couldn't open {}: {}", path, why ),
        Ok( file ) => file
    };
    for line in BufReader::new( file ).lines()
    {
        v.push( match line
        {
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
    for i in 1..depths.len()
    {
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
    for i in 3..depths.len()
    {
        if( depths[i] + depths[i - 1] + depths[i - 2] > depths[i - 1] + depths[i - 2] + depths[i - 3] )
        {
            num_increasing += 1;
        }
    }
    return num_increasing.to_string();
}
