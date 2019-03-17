
extern crate cluuname;

use cluuname::UtsElement;
use cluuname::UtsName;
use cluuname::uname;
use cluuname::build;

fn main() {
	let uname = uname().unwrap();
	nodename(&uname);
	//NODENAME "R510"

	let custom_uname = build::linux_216_86();
	nodename(&custom_uname);
	//NODENAME "cluComp"
	
}

#[cfg(feature = "enable_domainname")]
fn nodename<Q, W, E, R, T, Y>(uname: &UtsName<Q, W, E, R, T, Y>) where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
	println!("NODENAME {}", uname.display_nodename());
}

#[cfg(not(feature = "enable_domainname"))]
fn nodename<Q, W, E, R, T>(uname: &UtsName<Q, W, E, R, T>) where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	println!("NODENAME {}", uname.display_nodename());
}
