

/*!
Slice and boxed uname structures
*/

///Boxed UtsName
mod buf;

///Slice UtsName
mod slice;

pub use self::buf::*;
pub use self::slice::*;

///#[cfg(feature = "enable_domainname")]
#[cfg(feature = "enable_domainname")]
#[allow(dead_code)]
pub const ENABLE_DOMAIN_NAME: bool = true;

///#[cfg(feature = "enable_domainname")]
#[cfg(not(feature = "enable_domainname"))]
#[allow(dead_code)]
pub const ENABLE_DOMAIN_NAME: bool = false;
