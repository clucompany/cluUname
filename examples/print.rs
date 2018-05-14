

extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;


pub fn main() {
	let uname = uname().unwrap();
	
	println!("{} {} {} {} {}",
		uname.display_sysname(),
		uname.display_nodename(),
		uname.display_release(),
		uname.display_version(),
		uname.display_machine(),
	);
}
