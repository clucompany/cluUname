
use cluuname::core::UnameErr;
use cluuname::rs_uname;
use cluuname::uname;

fn main() -> Result<(), UnameErr> {
	// #1
	let a_uname = rs_uname()?; // cowstring uname's
	println!("sysname: {}", a_uname.get_str_sysname());
	println!("nodename: {}", a_uname.get_str_nodename());
	println!("release: {}", a_uname.get_str_release());
	println!("version: {}", a_uname.get_str_version());
	println!("machine: {}", a_uname.get_str_machine());
	println!("domainname: {}", a_uname.get_str_domainname());
	
	println!("{}", a_uname);
	
	// #2
	let a_uname = uname()?; // def rawarray uname's
	println!("sysname: {}", String::from_utf8_lossy(a_uname.get_cstr_sysname().to_bytes()));
	println!("{}", a_uname.get_slow_display());
	
	Ok(())
}
