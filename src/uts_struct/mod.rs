

/*!
Slice and boxed uname structures
*/

///Boxed UtsName
pub mod buf;

///Slice UtsName
pub mod slice;


///#[cfg(feature = "enable_domainname")]
#[cfg(feature = "enable_domainname")]
pub const ENABLE_DOMAIN_NAME: bool = true;

///#[cfg(feature = "enable_domainname")]
#[cfg(not(feature = "enable_domainname"))]
pub const ENABLE_DOMAIN_NAME: bool = false;
