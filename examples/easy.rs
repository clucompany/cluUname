
use cluuname::rs_uname;
use cluuname::uname;

fn main() {
	let a_uname = rs_uname(); // string uname's
	println!("{}", a_uname); // a_uname.display() <- fast
	
	let a_uname = uname(); // def rawarray uname's
	println!("{}", a_uname.get_slow_display());
}
