
extern crate cluuname;
use cluuname::build::custom;


pub fn main() {
	let hash_version_test = custom (
		"Linux",
		&Some("cluComp"),
		(),	// <<<
		"#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008", // <<<
		"x86",
			
		#[cfg(feature = "enable_domainname")]
		"(none)",
	);
	
	
	println!("{} {} {} {} {}",
		hash_version_test.as_sysname(),
		hash_version_test.as_nodename(),
		hash_version_test.as_release(),
		hash_version_test.as_version(),
		hash_version_test.as_machine(),
	);
	
}
