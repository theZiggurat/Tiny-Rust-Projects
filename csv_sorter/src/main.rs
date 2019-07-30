fn main() {
    
    // use clap to configure application arguments
    use clap::{Arg, App};
    let matches = App::new("csv_sorter")
        .arg(Arg::with_name("col_name")
            .short("c")
            .long("col_name")
            .value_name("COLUMN")
            .help("Column to sort over")
            .required(true))
        .arg(Arg::with_name("d")
                .short("d")
                .long("descening")
                .help("Sort in descending order"))
        .get_matches();

    let desc = if matches.is_present("d") {
        true
    } else {
        false
    };

    // get column name from clap. this is what we will sort over
    let col: String = matches.value_of("col_name").unwrap().to_string();

    // read stdin to buf
    use std::io::{self, Read};
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)
        .expect("Could not read file, make sure to pipe it to stdin");
    
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

    enum Type {
        Numeric,
        Str
    }

    // see if column were sorting over is numeric or string
    let t = match rows.get(0).unwrap().get(index).unwrap().parse::<f64>() {
        Ok(_) => Type::Numeric,
        Err(_) => Type::Str
    };

    rows.sort_by(|a, b| {
        let res = match t {
            Type::Numeric => {
                let t1 = a.get(index).unwrap().parse::<f64>().unwrap_or(0f64);
                let t2 = b.get(index).unwrap().parse::<f64>().unwrap_or(0f64);
                t1.partial_cmp(&t2).unwrap()
            },
            Type::Str => {
                a.get(index).unwrap().cmp(b.get(index).unwrap())
            }
        };

        if desc {
            res.reverse()
        } else {
            res
        }
    });

    // gross non-functional blocks for printing code to stdout
    for (i, s) in records.iter().enumerate() {
        if i == records.len() - 1 {
            println!("{}", s);
        } else {
            print!("{},", s);
        }
    }

    for r in rows {
        for (i, s) in r.iter().enumerate() {
            if i == r.len() - 1 {
                println!("{}", s);
            } else {
                print!("{},", s);
            }
        }
    }
}
