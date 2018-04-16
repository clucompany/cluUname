

/*!
Slice and boxed structures
*/

///Boxed UtsName
pub mod buf;

///Slice UtsName
pub mod slice;


///Boxed UtsName
pub type UtsBuf = self::buf::UtsNameBuf;

///Slice UtsName
pub type UtsSlice<'a> = self::slice::UtsNameSlice<'a>;

///#[cfg(feature = "enable_domainname")]
#[cfg(feature = "enable_domainname")]
pub const ENABLE_DOMAIN_NAME: bool = true;

///#[cfg(feature = "enable_domainname")]
#[cfg(not(feature = "enable_domainname"))]
pub const ENABLE_DOMAIN_NAME: bool = false;
