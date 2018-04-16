

/*!
Slice and boxed structures
*/


pub mod buf;
pub mod slice;


///Boxed UtsName
pub type UtsBuf = self::buf::UtsNameBuf;

///Slice UtsName
pub type UtsSlice<'a> = self::slice::UtsNameSlice<'a>;

