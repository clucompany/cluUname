
extern crate cluuname;
use cluuname::uname;

pub fn main() {
	let uname = uname().unwrap();
	
	println!("{}", uname);
}
