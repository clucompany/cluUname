
use crate::Uname;
use core::fmt::Display;
use core::fmt::Debug;
use core::num::NonZeroI32;

pub trait UnameData where Self: Sized + Debug {
	type Data;
	
	fn empty_data() -> Self::Data;
	fn get_current_fn<R>(
		ok: impl FnOnce(Uname<Self>) -> R, 
		e: impl FnOnce(UnameErr) -> R
	) -> R;
	
	#[inline(always)]
	fn get_current() -> Result<Uname<Self>, UnameErr> {
		Self::get_current_fn(
			|u| Ok(u),
			|e| Err(e)
		)
	}
	
	#[inline(always)]
	fn get_current_or_empty() -> Uname<Self> {
		Self::get_current_fn(
			|u| u,
			|_| Self::from_data(Self::empty_data()),
		)
	}
	
	fn hash_data<H: ::core::hash::Hasher>(data: &Self::Data, state: &mut H);
	fn from_data(data: Self::Data) -> Uname<Self>;
}

#[derive(Debug)]
pub enum UnameErr {
	LibcErr(NonZeroI32)
}

impl Display for UnameErr {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::LibcErr(e) => write!(f, "`libc::uname` returned error {}.", e),
		}
	}
}

pub trait AsUname<T> where T: ?Sized {
	type Data;
	
	fn as_sysname<'a>(data: &'a Self::Data) -> &'a T;
	fn as_nodename<'a>(data: &'a Self::Data) -> &'a T;
	fn as_release<'a>(data: &'a Self::Data) -> &'a T;
	fn as_version<'a>(data: &'a Self::Data) -> &'a T;
	fn as_machine<'a>(data: &'a Self::Data) -> &'a T;
	fn as_domainname<'a>(data: &'a Self::Data) -> &'a T;
}

pub trait AsPtrUname<T> where T: ?Sized {
	type Data;
	
	fn as_ptr_sysname(data: &Self::Data) -> *const T;
	fn as_ptr_nodename(data: &Self::Data) -> *const T;
	fn as_ptr_release(data: &Self::Data) -> *const T;
	fn as_ptr_version(data: &Self::Data) -> *const T;
	fn as_ptr_machine(data: &Self::Data) -> *const T;
	fn as_ptr_domainname(data: &Self::Data) -> *const T;
}
