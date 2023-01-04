
//! The basic set of features used in the "uname" structure and in 
//! describing the behavioral features of "uname".

use crate::Uname;
use core::fmt::Display;
use core::fmt::Debug;
use core::num::NonZeroI32;

/// A trait that defines the behavior of the `Uname` structure.
pub trait UnameBeh where Self: Sized + Debug {
	/// The data that is stored inside the `Uname` structure.
	type Data;
	
	/// Create empty data for use inside the `Uname` structure.
	fn build_empty_data() -> Self::Data;
	
	/// Create a `Uname` structure, `ok` on success, `err` on failure.
	fn get_current_fn<R>(
		ok: impl FnOnce(Uname<Self>) -> R, 
		e: impl FnOnce(UnameErr) -> R
	) -> R;
	
	/// Create a structure `Uname`, `Result::Ok` on success, `Result::Err` on failure.
	#[inline(always)]
	fn get_current() -> Result<Uname<Self>, UnameErr> {
		Self::get_current_fn(
			|u| Ok(u),
			|e| Err(e)
		)
	}
	
	/// Create a `Uname` struct, return a filled `Uname` on success, 
	/// return an empty unfilled `Uname` on failure.
	#[inline(always)]
	fn get_current_or_empty() -> Uname<Self> {
		Self::get_current_fn(
			|u| u,
			|_| Self::from_data(Self::build_empty_data()),
		)
	}
	
	/// Like `Hash::hash`, but for use in conjunction with the `Uname` structure. 
	/// (note that the `Hash::hash` trait is described for `Uname`).
	fn hash_data<H: ::core::hash::Hasher>(data: &Self::Data, state: &mut H);
	
	/// Create a `Uname` structure using arbitrary data.
	fn from_data(data: Self::Data) -> Uname<Self>;
}

/// An enumeration describing errors that occur when getting `libc::uname`.
#[repr(i32)]
#[derive(Debug)]
pub enum UnameErr {
	/// The error number received from the `libc::uname` function 
	/// (note that this is NonZero, since 0 means success, 
	/// and a non-zero value causes this error).
	LibcErr(NonZeroI32)
}

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg( any(test, feature = "std") )]
impl std::error::Error for UnameErr {
	#[inline]
	fn description(&self) -> &str {
		match self {
			Self::LibcErr(..) => "`libc::uname` returned invalid error.",
		}
	}
}

impl Display for UnameErr {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::LibcErr(e) => write!(f, "`libc::uname` returned error {}.", e),
		}
	}
}

/// A trait describing the type of data that can be obtained from the `uname` 
/// behavior for the `uname` structure.
pub trait GetUname<T> where T: ?Sized {
	/// The data type used in the `uname` structure is 
	/// associated with the `uname` behavior data trait.
	type Data;
	
	/// Get a reference to the `sysname` data 
	/// (data conversion is possible when calling this function or when creating 
	/// a `Uname` structure, see the uname behavior you are using).
	fn get_sysname<'a>(data: &'a Self::Data) -> &'a T;
	/// Get a reference to the `nodename` data 
	/// (data conversion is possible when calling this function or when creating 
	/// a `Uname` structure, see the uname behavior you are using).
	fn get_nodename<'a>(data: &'a Self::Data) -> &'a T;
	/// Get a reference to the `release` data 
	/// (data conversion is possible when calling this function or when creating 
	/// a `Uname` structure, see the uname behavior you are using).
	fn get_release<'a>(data: &'a Self::Data) -> &'a T;
	/// Get a reference to the `version` data 
	/// (data conversion is possible when calling this function or when creating 
	/// a `Uname` structure, see the uname behavior you are using).
	fn get_version<'a>(data: &'a Self::Data) -> &'a T;
	/// Get a reference to the `machine` data 
	/// (data conversion is possible when calling this function or when creating 
	/// a `Uname` structure, see the uname behavior you are using).
	fn get_machine<'a>(data: &'a Self::Data) -> &'a T;
	/// Get a reference to the `domainname` data 
	/// (data conversion is possible when calling this function or when creating 
	/// a `Uname` structure, see the uname behavior you are using).
	fn get_domainname<'a>(data: &'a Self::Data) -> &'a T;
}

/// The trait that describes a data type as it is, without casting to any data type, 
/// is implemented by the `uname` behavior for the `uname` structure.
pub trait AsPtrUname<T> where T: ?Sized {
	/// The data type used in the `uname` structure is 
	/// associated with the `uname` behavior data trait.
	type Data;
	
	/// Get a pointer to the `sysname` data (as is, no conversion).
	fn as_ptr_sysname(data: &Self::Data) -> *const T;
	/// Get a pointer to the `nodename` data (as is, no conversion).
	fn as_ptr_nodename(data: &Self::Data) -> *const T;
	/// Get a pointer to the `release` data (as is, no conversion).
	fn as_ptr_release(data: &Self::Data) -> *const T;
	/// Get a pointer to the `version` data (as is, no conversion).
	fn as_ptr_version(data: &Self::Data) -> *const T;
	/// Get a pointer to the `machine` data (as is, no conversion).
	fn as_ptr_machine(data: &Self::Data) -> *const T;
	/// Get a pointer to the `domainname` data (as is, no conversion).
	fn as_ptr_domainname(data: &Self::Data) -> *const T;
}
