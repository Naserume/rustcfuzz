use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() {
    let v : Vec<String> = env::args().collect();
    if v.len() == 2 {
	read_file(&v[1]);
    }
    else {
	panic!("wrong number of arguments");
    }
}

fn read_file(file_name: &String) {
    let file = fs::File::open(file_name).expect("no file with this name");
    let x : BTreeMap<String, serde_yaml::Value> =
	serde_yaml::from_reader(file).expect("parse error");

    println!("{:?}", x);
    println!("{:?}", x["constants"]);
}

fn parse_constants(x: &BTreeMap<String, serde_yaml::Value>)
		   -> BTreeMap<String,String>
{
    let c = |(x, y): (&String, &String)| {
	f((x, y)).expect("failed to parse")
    };

    x.iter().map(c).collect()
}

fn f((x,y): (&String, &serde_yaml::Value)) -> Result<(String, String), String>{
    match y {
	serde_yaml::Value::String(s) => Ok ((x.clone(), s.clone())),
	_ => Err(String::new())
    }
}
