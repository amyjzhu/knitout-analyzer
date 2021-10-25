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
use std::collections::HashMap;
use std::path::Path;
use std::convert::TryFrom;
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum KnitInstType {
    Knit,
    Tuck,
    Inhook,
    Outhook,
    Releasehook,
    Drop,
    XStitchNumber,
    Rack,
    Transfer,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum KnitBed {
    Front,
    Back,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
enum KnitDir {
    Forward,
    Backward
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct Needle {
    bed: KnitBed,
    num: i64,
}

#[derive(Debug)]
#[derive(Clone)]
struct KnitInst {
    inst_type : KnitInstType,
    needle: Option<Needle>,
    needle2: Option<Needle>,
    direction: Option<KnitDir>,
    carrier: Option<u64>,
    offset: Option<i64>,
}

struct KnitProgram {
    row_insts: Vec<Vec<KnitInst>>,
    needle_insts: HashMap<Needle, Vec<KnitInst>>,
    program: Vec<KnitInst>
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

fn get_knit_type(inst: &String) -> Result<KnitInstType, String> {
    match inst.as_str() {
        "knit" => Ok(KnitInstType::Knit),
        "tuck" => Ok(KnitInstType::Tuck),
        "inhook" => Ok(KnitInstType::Inhook),
        "outhook" => Ok(KnitInstType::Outhook),
        "releasehook" => Ok(KnitInstType::Releasehook),
        "x-stitch-number" => Ok(KnitInstType::XStitchNumber),
        "rack" => Ok(KnitInstType::Rack),
        "drop" => Ok(KnitInstType::Drop),
        "xfer" => Ok(KnitInstType::Transfer),
        _ => Err(["No matching inst type found: ".to_string(), inst.to_string()].join("")), 
    }
}

// todo: error macro?
fn get_knit_dir(dir: &String) -> Result<KnitDir, String> {
    return match dir.as_str() {
        "+" => Ok(KnitDir::Forward),
        "-" => Ok(KnitDir::Backward),
        _ => Err(["Direction cannot be parsed: ".to_string(), dir.to_string()].join("")),
    };
}

fn parse_needle_inst(needle: &String) -> Needle {
    let (bed_s, num) = needle.split_at(1);
    let bed = match bed_s {
        "f" => KnitBed::Front,
        "b" => KnitBed::Back, // what to do here if it's wrong?
        _ => panic!("Can't find suitable bed")
    };

    // TODO needles can be negative????
    return Needle { bed: bed, num: num.parse::<i64>().expect("Invalid needle number") };
}

/// ====
fn parse_knit_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Knit,
        needle: Some(parse_needle_inst(&all_params[2])),
        needle2: None,
        direction: Some(get_knit_dir(&all_params[1]).expect("Invalid direction")),
        carrier: Some(all_params[3].parse::<u64>().expect("Invalid carrier number")),
        offset: None,
    };
    return inst;
}

fn parse_tuck_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Tuck,
        needle: Some(parse_needle_inst(&all_params[2])),
        needle2: None,
        direction: Some(get_knit_dir(&all_params[1]).expect("Invalid direction")),
        carrier: Some(all_params[3].parse::<u64>().expect("Invalid carrier number")),
        offset: None,
    };
    return inst;
}

fn parse_drop_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Drop,
        needle: Some(parse_needle_inst(&all_params[1])),
        needle2: None,
        direction: None,
        carrier: None,
        offset: None,
    };
    return inst;
}

fn parse_inhook_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Inhook,
        needle: None,
        needle2: None,
        direction: None,
        carrier: Some(all_params[1].parse::<u64>().expect("Invalid carrier number")),
        offset: None,
    };
    return inst;
}

fn parse_outhook_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Releasehook,
        needle: None,
        needle2: None,
        direction: None,
        carrier: Some(all_params[1].parse::<u64>().expect("Invalid carrier number")),
        offset: None,
    };
    return inst;
}

fn parse_releasehook_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Releasehook,
        needle: None,
        needle2: None,
        direction: None,
        carrier: Some(all_params[1].parse::<u64>().expect("Invalid carrier number")),
        offset: None,
    };
    return inst;
}

fn parse_xstitchnumber_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::XStitchNumber,
        needle: None,
        needle2: None,
        direction: None,
        carrier: None,
        offset: Some(all_params[1].parse::<i64>().expect("Invalid carrier number")),
        // TODO: not actually an offset but a stitch number! diff. setting. make better sol'n here
    };
    return inst;
}

fn parse_rack_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Rack,
        needle: None,
        needle2: None,
        direction: None,
        carrier: None,
        offset: Some(all_params[1].parse::<i64>().expect(&format!("Invalid carrier number: {}", all_params[1]))),
    };
    return inst;
}

fn parse_transfer_inst(all_params: Vec<String>) -> KnitInst {
    let inst = KnitInst {
        inst_type: KnitInstType::Transfer,
        needle: Some(parse_needle_inst(&all_params[1])),
        needle2: Some(parse_needle_inst(&all_params[2])),
        direction: None,
        carrier: None,
        offset: None,
    };
    return inst;
}

fn get_knit_parse_proc(inst: KnitInstType) -> Result<Box<dyn Fn(Vec<String>) -> KnitInst>, String> {
    match inst {
        KnitInstType::Knit => return Ok(Box::new(parse_knit_inst)),
        KnitInstType::Tuck => return Ok(Box::new(parse_tuck_inst)),
        KnitInstType::Drop => return Ok(Box::new(parse_drop_inst)),
        KnitInstType::Inhook => return Ok(Box::new(parse_inhook_inst)),
        KnitInstType::Outhook => return Ok(Box::new(parse_outhook_inst)),
        KnitInstType::Releasehook => return Ok(Box::new(parse_releasehook_inst)),
        KnitInstType::XStitchNumber => return Ok(Box::new(parse_xstitchnumber_inst)),
        KnitInstType::Rack => return Ok(Box::new(parse_rack_inst)),
        KnitInstType::Transfer => return Ok(Box::new(parse_transfer_inst)),
            // |x| KnitInst { inst_type: KnitInstType::Knit, needle: 0, bed: KnitBed::Front}),
         _ => 
        //  return Ok(Box::new(|x| KnitInst {
        //     inst_type: KnitInstType::Releasehook,
        //     needle: None,
        //     direction: None,
        //     carrier: None,}))
        Err("No such procedure".to_string()),
    }
}

// todo we should probably have a procedure associated with each inst
// like a strategy
fn main() {
    let k_program_filename = "./pleated2.k";
    // let k_program_filename = "./intarsia.knitout";
    //let k_program = KnitProgram {}

    let mut all_insts = Vec::<KnitInst>::new();
    // this doesn't seem quite right... just knowing the needles doesn't mean knowing rows
    let mut program_by_rows = Vec::<Vec<KnitInst>>::new();

    if let Ok(lines) = read_lines(k_program_filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                // just add it to the row for the simplest version
                // split on spaces, check the first
                // also check for ;;
                let inst = l.split(" ");
                // should not iterate, should just look at first
                let inst_pieces: Vec<String> = inst.map(|s| s.to_string()).collect();
                // check to see if it starts with ; or ;;
                let mut directives = Vec::<String>::new();
                if inst_pieces[0].starts_with(";") {
                    directives.push(l);
                    println!("Got directive");
                    
                } else {
                    let knit_type = get_knit_type(&inst_pieces[0]).expect("No matching inst found!");
                    let parsed = get_knit_parse_proc(knit_type).expect("No matching procedure")(inst_pieces);

                    // NOW: we need to sort by row and by needle number
                    // let's use our map to put needle number
                    // and we can follow switches of fd and bk to get rows 
                    
                    // first we find the needle number


                    // match parsed.needle {
                    //     Some(needle) => None,
                    //     None => None,
                    // };

                    //println!("{:?}", &parsed);
                    all_insts.push(parsed);
                    
                }
                
            }
        }
    }

    // first one may not have direction
    let mut current_dir : Option<KnitDir> = None;
    let mut current_row = Vec::<KnitInst>::new();
    // now need to sort row by row
    for inst in all_insts.clone() {
        // would using an iterator + fold avoid borrowing? just do first/rest
        // cloning is a terrible solution
        // todo also make this a separate function
        match inst.direction {
            Some(dir) => {
                match current_dir {
                    Some(cur_dir) => {
                    if dir == cur_dir {
                        current_row.push(inst);
                    } else {
                        program_by_rows.push(current_row);
                        current_row = Vec::<KnitInst>::new();
                        current_row.push(inst);
                        current_dir = Some(dir);
                    }
                    },
                    None => current_dir = Some(dir),
                    // current dir is None means it's first time
                }
            },

            None => current_row.push(inst), // just leave it...
            }
        };

    program_by_rows.push(current_row);

    println!("{:?}", program_by_rows);

    let mut by_needle_insts = HashMap::<Needle, Vec<KnitInst>>::new();

    // make the by-needle thing
    for inst in all_insts.clone() {
        match inst.needle {
            Some(ref needle) => {
                by_needle_insts.entry(*needle)
                // If there's no entry for key 3, create a new Vec and return a mutable ref to it
                .or_default()
                // and insert the item onto the Vec
                .push(inst); 
            },
            None => (),
            // meh
        }
        
    }


    let program = KnitProgram { program: all_insts, row_insts: program_by_rows, needle_insts: by_needle_insts };

    count_transfers(program);
}

fn count_transfers(program: KnitProgram) -> usize {
    let all_instructions = program.program;

    let count = all_instructions.iter().filter(|x| x.inst_type == KnitInstType::Transfer).count();

    println!("{}", count);

    return count;

}

// how do we actually calculate slack? need to read 
// construct loop states 

#[derive(Copy)]
#[derive(Clone)]
struct Loop {
    id: i64,
}


struct LoopState {
    // loops are on which needles? 
    // what will be the easiest way to check slack?
    // order loops by needles... 
    // make a map... and then... ahh
    // loop through it
    loops: HashMap<Needle, Loop>,
    last_loop_id : i64,
    racking: i64, // could be negative!
    max_slack: u64,
    max_slack_achieved_at_loop: Loop,
    max_slack_achieved_at_needle: Needle,
}


// if we rack, we should consider slack
// slack changes mainly when we move needles 
// check needle index (i.e. doesn't change from xfer front to back)
// probably all in the racking then?

// TODO: do we need to consider the whole state rather than just the curent state?

fn construct_loop_state(mut loop_state: LoopState, inst: KnitInst) -> LoopState { // return new loopstate
    // TODO: do we have to worry about carrier here?
    match inst.inst_type {
        KnitInstType::Tuck => {
            let id = loop_state.last_loop_id + 1;
            loop_state.last_loop_id = id;

            loop_state.loops.insert(
            inst.needle.expect("Why does the tuck not have a needle"),
            Loop {id: id});
            return loop_state;
        },// add a new loop to that needle
        KnitInstType::Transfer => {
            // move the loop to its new location
            // calculate slack (taking racking into account)
            // set max slack if necessary 
            let needle1 = inst.needle.expect("Transfer has no 1st needle");
            let needle2 = inst.needle.expect("Transfer has no 2nd needle");

            // calculate the distance between the needles
            // racking is back bed moving w.r.t the front (ensure which direction)
            let racking_factor = 1;

            let racking = loop_state.racking;

            let diff = needle1.num - needle2.num;
            let slack: u64 = u64::try_from((racking_factor * racking + diff).abs()).unwrap(); 

            if slack > loop_state.max_slack {
                loop_state.max_slack = slack;
                loop_state.max_slack_achieved_at_loop = *loop_state.loops.get(&needle1).expect("No loop on needle?");
                loop_state.max_slack_achieved_at_needle = needle1;
            }

            return loop_state;
        }, //move the loop around
        KnitInstType::Drop => {
            loop_state.loops.remove(&inst.needle.expect("Why does the drop not have a needle"));
            return loop_state;
        }, // get rid of the loop,
        KnitInstType::Rack => {
            // set a new rack value (this affects slack calculations)
            loop_state.racking = inst.offset.expect("There is no rack value");
            return loop_state;
        }
        _ => loop_state,
    }
}

// TODO: need to consider current rack status
// TODO: 


// once we have a list of loop states, we can calculate the slack between each needle
// where are the loops? 
// how is slack calculated?? is it loops or needles? eee