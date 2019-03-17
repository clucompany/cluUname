
extern crate cluuname;
use cluuname::UtsElement;
use cluuname::uname;

pub fn main() {
	let uname = uname().unwrap();
	
	let [
		sysname,
		_nodename,
		_release,
		_version,
		_machine, //&CString
	] = uname.as_array0();
	
	println!("{}", sysname as &dyn UtsElement);
}
