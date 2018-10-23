
extern crate cluuname;

use cluuname::UtsName;
use cluuname::uname;
use cluuname::build;

fn main() {
	let uname = uname().unwrap();
	nodename(&uname);
	nodename(uname);
	//NODENAME "R510"

	let custom_uname = build::linux_216_86();
	nodename(custom_uname);
	//NODENAME "cluComp"
}

fn nodename<T: UtsName>(uname: T) {
	println!("NODENAME {}", uname.display_nodename());
}