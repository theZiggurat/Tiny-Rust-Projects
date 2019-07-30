use std::io::prelude::*;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::time::Instant;

fn main() {
    
    // use clap to configure application arguments
    use clap::{Arg, App};
    let matches = App::new("csv_sorter")
        .arg(Arg::with_name("file_name")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("CSV file to sort")
            .required(true))
        .arg(Arg::with_name("col_name")
            .short("c")
            .long("col_name")
            .value_name("COLUMN")
            .help("Column to sort over")
            .required(true))
        .arg(Arg::with_name("desc")
                .short("d")
                .long("descening")
                .help("Sort in descending order"))
        .get_matches();

    // determine if sorting is descending or ascending
    let desc = if matches.is_present("desc") { true} 
               else { false };

    // open file by getting file name from clap arguments
    let file_name = matches.value_of("file_name").unwrap().to_string();
    let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(file_name)
                    .expect("Cannot open file");

    // get column name from clap. this is what we will sort over
    let col: String = matches.value_of("col_name").unwrap().to_string();

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Could not read file");
    
    // get all csv records from first line of input
    let records = buf.clone().lines()       // split lines
        .nth(0)                             // take first line 
        .expect("No content in input file") // if no first line, panic
        .split(',')                         // split on comma
        .map(|s| s.to_string())             // map from &str to String
        .collect::<Vec<_>>();               // collect in Vec<String>

    // find index of input csv column, panic if none of them are equal
    let index  = match records.iter()       // match on ------------------------------|
            .enumerate()                    // zip with index                         |
            .filter(|(_, s)| **s == col)    // find all matching input col name       |
            .nth(0)                         // return first as tuple (index, string)<-|
    {
        Some((i, _)) => i,                  // return index from tuple
        None => panic!("Input column name does not match any of the input records")
    };

    // get rest of lines (excluding records line) in a vector
    let mut rows: Vec<_> = buf.lines()
        .skip(1)
        .map(|s| s.to_string()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>())
        .collect();

    // theres two types a csv entry can be
    enum Type {
        Numeric,
        Str
    }

    // see if column were sorting over is numeric or string
    let t = match rows.get(0).unwrap().get(index).unwrap().parse::<f64>() {
        Ok(_) => Type::Numeric,
        Err(_) => Type::Str
    };

    let time = Instant::now();
    rows.par_sort_by(|a, b| {
        // get data entries from row by indexing it
        let a = a.get(index).unwrap();
        let b = b.get(index).unwrap();

        // obtain an ordering
        let res = match t {
            Type::Numeric => {
                let t1 = a.parse::<f64>().unwrap_or(0f64);
                let t2 = b.parse::<f64>().unwrap_or(0f64);
                t1.partial_cmp(&t2).unwrap()
            },
            Type::Str => a.cmp(b)
        };

        // post-process ordering
        if desc { res.reverse() } 
        else { res }
    });
    println!("Sorting finished. Took {} nanoseconds", time.elapsed().as_nanos());
    println!("Writing to file...");

    // clear file so we can write
    let _ = file.set_len(0);

    // gross non-functional blocks for writing sorted csv to file
    let err: &'static str = "Error writing to file";
    for (i, s) in records.iter().enumerate() {
        if i == records.len() - 1 {
            writeln!(file, "{}", s).expect(err);
        } else {
            write!(file, "{},", s).expect(err);
        }
    } for r in rows {
        for (i, s) in r.iter().enumerate() {
            if i == r.len() - 1 {
                writeln!(file, "{}", s).expect(err);
            } else {
                write!(file, "{},", s).expect(err);
            }
        }
    }
    println!("Finished!");
}
