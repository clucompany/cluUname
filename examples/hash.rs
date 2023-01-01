
use cluuname::core::UnameErr;
use cluuname::rs_uname;
use cluuname::uname;

fn main() -> Result<(), UnameErr> {
	let a_uname = rs_uname()?; // cowstring uname's
	println!("sysname: {}", a_uname.as_str_sysname());
	println!("{}", a_uname);
	
	let a_uname = uname()?; // def rawarray uname's
	println!("{}", a_uname.get_slow_display());
	
	Ok(())
}

/* +feature = "std"
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let a_uname = rs_uname()?; // string uname's
	println!("{}", a_uname); // a_uname.display() <- fast
	
	let a_uname = uname()?; // def rawarray uname's
	println!("{}", a_uname.get_slow_display());
	
	Ok(())
}
*/
