
use crate::element::UtsElementIntoUTF;
use crate::element::UTSUTF8Err;
use crate::uts_struct::UtsNameUTF8;
use crate::uts_struct::UtsNameAlwaysType;
use crate::hash::HashVersion;
use std::hash::Hasher;
use std::fmt::Display;
use crate::display::DisplayUts;
use std::hash::Hash;
use crate::element::UtsElement;
use std::fmt;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UtsName<Q, W, E, R, T, Y>  where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement  {
	sysname:	Q,
	nodename:	W,
	release:	E,
	version:	R,
	machine:	T,
	
	domainname:	Y,
}

impl<Q, W, E, R, T, Y> From<(Q, W, E, R, T, Y)> for UtsName<Q, W, E, R, T, Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
	#[inline(always)]
	fn from((q, w, e, r, t, y): (Q, W, E, R, T, Y)) -> Self {
		Self::new(q, w, e, r, t, y)	
	}
}

impl<'a, Q:'a, W:'a, E:'a, R:'a, T:'a, Y: 'a> UtsName<Q, W, E, R, T, Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement ,Y: UtsElement {	
	#[inline(always)]
	pub const fn as_sysname(&self) -> &(dyn UtsElement + 'a) {
		&self.sysname
	}
	
	#[inline(always)]
	pub const fn as_nodename(&self) -> &(dyn UtsElement + 'a) {
		&self.nodename
	}
	
	#[inline(always)]
	pub const fn as_release(&self) -> &(dyn UtsElement + 'a) {
		&self.release
	}
	
	#[inline(always)]
	pub const fn as_version(&self) -> &(dyn UtsElement + 'a) {
		&self.version
	}
	
	#[inline(always)]
	pub const fn as_machine(&self) -> &(dyn UtsElement + 'a) {
		&self.machine
	}
	
	
	
	pub fn uname_hash(&self) -> u64 where Q: Hash, W: Hash, E: Hash, R: Hash, T: Hash, Y: Hash {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);

		hasher.finish()  
	}
	
	pub fn version_hash(&self) -> u64 where Q: Hash, E: Hash, R: Hash {		
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash_version(&mut hasher);
		
		hasher.finish()  
	}
	
	#[inline]
	///Display trait for sysname.
	pub const fn display_sysname(&self) -> DisplayUts<&Q> {
		DisplayUts::new(&self.sysname)
	}
	
	///Display trait for nodename.
	#[inline]
	pub const fn display_nodename(&self) -> DisplayUts<&W> {
		DisplayUts::new(&self.nodename)
	}
	
	///Display trait for release.
	#[inline]
	pub const fn display_release(&self) -> DisplayUts<&E> {
		DisplayUts::new(&self.release)
	}
	
	///Display trait for version.
	#[inline]
	pub const fn display_version(&self) -> DisplayUts<&R> {
		DisplayUts::new(&self.version)
	}
	
	///Display trait for machine.
	#[inline]
	pub const fn display_machine(&self) -> DisplayUts<&T> {
		DisplayUts::new(&self.machine)
	}
	
	#[inline]
	pub const fn new(q:Q, w:W, e:E, r:R, t:T, y:Y) -> Self {
		Self {
			sysname:	q,
			nodename:	w,
			release:	e,
			version:	r,
			machine:	t,
			
			domainname:	y,
		}
	}
	
	#[inline(always)]
	pub const fn as_domainname(&self) -> &(dyn UtsElement + 'a) {
		&self.domainname
	}
	
	///Display trait for domainname.
	#[inline]
	pub const fn display_domainname(&self) -> DisplayUts<&Y> {
		DisplayUts::new(&self.domainname)
	}
	
	pub fn as_bytes(&self) -> [&[u8]; 6] {
		[
			self.sysname.as_array(),
			self.nodename.as_array(),
			self.release.as_array(),
			self.version.as_array(),
			self.machine.as_array(),
			
			self.domainname.as_array(),
		]
	}
	
	pub fn as_bytes0(&self) -> [&[u8]; 5] {
		[
			self.sysname.as_array(),
			self.nodename.as_array(),
			self.release.as_array(),
			self.version.as_array(),
			self.machine.as_array(),
		]
	}
	
	pub fn to_utf8(self) -> Result<UtsNameUTF8, UTSUTF8Err> where Q: UtsElementIntoUTF, W: UtsElementIntoUTF, E: UtsElementIntoUTF, R: UtsElementIntoUTF, T: UtsElementIntoUTF, Y: UtsElementIntoUTF {
		Ok({			
			UtsNameUTF8::new(
				self.sysname.into_utf8()?,
				self.nodename.into_utf8()?,
				self.release.into_utf8()?,
				self.version.into_utf8()?,
				self.machine.into_utf8()?,
				
				self.domainname.into_utf8()?,
			)	
		})
	}
}


impl<Q,W,E,R,T,Y> Display for UtsName<Q, W, E, R, T, Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, 
			"{} {} {} {} {} {}",
			
			self.as_sysname(),
			self.as_nodename(),
			self.as_release(),
			self.as_version(),
			self.as_machine(),
			
			self.as_domainname(),
		)
	}
}

impl<Q, W, E, R, T, Y> HashVersion for UtsName<Q, W, E, R, T, Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.sysname.as_array().hash(state);
		self.release.as_array().hash(state);
		self.version.as_array().hash(state);
		
		//HASH CSTR != HASH STR
	}
}

impl<Q, W, E, R, T, Y> Hash for UtsName<Q, W, E, R, T, Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.sysname.as_array().hash(state);
		self.nodename.as_array().hash(state);
		self.release.as_array().hash(state);
		self.version.as_array().hash(state);
		self.machine.as_array().hash(state);
		self.domainname.as_array().hash(state);
		
		//HASH CSTR != HASH STR
	}
}


impl<T> UtsNameAlwaysType<T> where T: UtsElement {
	#[inline]
	pub const fn as_array(&self) -> [&T; 6] {
		[
			&self.sysname,
			&self.nodename,
			&self.release,
			&self.version,
			&self.machine,
			
			&self.domainname,
		]
	}
	
	#[inline]
	pub const fn as_array0(&self) -> [&T; 5] {
		[
			&self.sysname,
			&self.nodename,
			&self.release,
			&self.version,
			&self.machine,
		]
	}
	
	#[inline]
	pub fn to_array(self) -> [T; 6] {
		[
			self.sysname,
			self.nodename,
			self.release,
			self.version,
			self.machine,
			
			self.domainname,
		]
	}
	
	#[inline]
	pub fn to_array0(self) -> [T; 5] {
		[
			self.sysname,
			self.nodename,
			self.release,
			self.version,
			self.machine,
		]
	}
}