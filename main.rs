// here's what we should do...
// maybe we need to make some kind of analyzer
// first parse into a structure
// what do we need to know? Needs to be a list of all instructions 
// considering the needle positions and so on...

// then we can add analysis passes onto it later

// should we just read it line by line to construct the data structure?
// we can determine which needle is being used by f[n] or b[n]

// why do we need + and - ? 
// I assume there's some kind of special operation where 
// moving direction actually matters and you can't tell by sequential
// should we store a run of + and -?
// or should we store per needle? hm
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Map;
use std::path::Path;

enum KnitInstType {
    Knit,
    Tuck,
}

enum KnitBed {
    Front,
    Back,
}

struct KnitInst {
    instType : KnitInstType,
    needle: u64,
    bed: KnitBed,
}

struct KnitProgram {
    rowInsts: Vec<KnitInst>,
    needleInsts: Map<u64, KnitInst>,
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

// todo we should probably have a procedure associated with each inst
fn main() {
    let k_program_filename = "./helloworld.k";
    // let k_program = KnitProgram {}

    if let Ok(lines) = read_lines(k_program_filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // just add it to the row for the simplest version

                println!("{}", ip);
            }
        }
    }
}

