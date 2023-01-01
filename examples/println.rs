
use cluuname::core::UnameErr;
use cluuname::rs_uname;
use cluuname::uname;

fn main() -> Result<(), UnameErr> {
	// #1
	let a_uname = rs_uname()?; // cowstring uname's
	println!("sysname: {}", a_uname.as_str_sysname());
	println!("nodename: {}", a_uname.as_str_nodename());
	println!("release: {}", a_uname.as_str_release());
	println!("version: {}", a_uname.as_str_version());
	println!("machine: {}", a_uname.as_str_machine());
	println!("domainname: {}", a_uname.as_str_domainname());
	
	println!("{}", a_uname);
	
	// #2
	let a_uname = uname()?; // def rawarray uname's
	println!("{}", a_uname.get_slow_display());
	
	Ok(())
}

/* +feature = "std"
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let a_uname = rs_uname()?; // string uname's
	println!("{}", a_uname);
	
	let a_uname = uname()?; // def rawarray uname's
	println!("{}", a_uname.get_slow_display());
	
	Ok(())
}
*/
