
//! An internal macro to create new behavior for the `uname` structure.

/// An internal macro to create new behavior for the `uname` structure.
#[doc(hidden)]
#[macro_export]
macro_rules! make_uname_data {
	// PUB OR PRIVATE (...)
	[
		$(#!for_enum: #[doc = $($doc:tt)*])*
		$(#![$cfg_ident:tt ( $($cfg:tt)* )])*
		#[$($addition_pub: tt)*] $enum_name: tt ( $($enum_data: tt)* ) {
			$( extern crate $externcrate_all:ident; )*
			$( use $use_all:path; )*
			
			$(#[ $($build_empty_data_addition: tt)* ])*
			build_empty_data || $(($build_empty_data_expr:expr))? $($build_empty_data:block)?, // EXPR || BLOCK
			
			$(#[ $($get_current_addition: tt)* ])*
			get_current	|$get_current_ok:ident, $get_current_e: ident| $(($get_current_expr:expr))? $($get_current:block)?, // EXPR || BLOCK
			
			$(
				#overload:
				$(#[ $($get_current_or_empty_addition: tt)* ])*
				get_current_or_empty|| $(($get_current_or_empty_expr:expr))? $($get_current_or_empty:block)?, // EXPR || BLOCK
			)?
			
			$(#[ $($from_addition: tt)* ])*
			from_data		|$data: ident| $(($from_expr:expr))? $($from:block)? , // EXPR || BLOCK
			
			$(#[ $($hash_addition: tt)* ])*
			hash_data|$hash_data:ident, $hash_state:ident| $(($hash_expr:expr))? $($hash:block)? $(,)?
			
			$(
				impl $impl_name:ident <$impl_arg:ty> for #$for: tt {
					$($impl_data:tt)*
				}
			)*
		} $(; $($unk:tt)* )? 
	] => {
		$(#[$cfg_ident( $($cfg)* )])*
		$crate::__next_macrocode_for_feature! {
			$( extern crate $externcrate_all; )*
			$( use $use_all; )*
			
			/*
				TODO, ONLY_RUSTCFG_DOC
			*/
			
			$(#[doc = $($doc)*])*
			$(#[$cfg_ident( $($cfg)* )])*
			#[derive(Debug)]
			$($addition_pub)* enum $enum_name {}
			
			$crate::__make_uname_structdata! {
				$(#[$cfg_ident( $($cfg)* )])*
				@make [$($addition_pub)*][ $($enum_data)* ]
			}
			
			$(#[$cfg_ident( $($cfg)* )])*
			impl $crate::UnameBeh for $enum_name {
				type Data = $crate::__make_uname_structdata!( @get_type[ $($enum_data)* ] );
				
				$(
					$(#[ $($get_current_or_empty_addition)* ])*
					fn get_current_or_empty() -> Uname<Self> where Self: Sized {
						$($get_current_or_empty_expr)?
						$($get_current_or_empty)?
					}
				)?
				
				$(#[ $($build_empty_data_addition)* ])*
				fn build_empty_data() -> Self::Data {
					$($build_empty_data_expr)?
					$($build_empty_data)?
				}
				
				fn get_current_fn<R>($get_current_ok: impl FnOnce(Uname<Self>) -> R, $get_current_e: impl FnOnce($crate::core::UnameErr) -> R) -> R {
					$($get_current_expr)?
					$($get_current)?
				}
				
				$(#[ $($from_addition)* ])*
				fn from_data($data: Self::Data) -> Uname<Self> where Self: Sized {
					$($from_expr)?
					$($from)?
				}
				
				$(#[ $($hash_addition)* ])*
				fn hash_data<H: ::core::hash::Hasher>($hash_data: &Self::Data, $hash_state: &mut H) {
					$($hash_expr)?
					$($hash)?
				}
			}
			
			$(#[$cfg_ident( $($cfg)* )])*
			$crate::__make_trait_for_uname! {
				@[ $enum_name ]: 
				$(
					impl $impl_name <$impl_arg> for #$for {
						$($impl_data)*
					}
				)*
			}
		}
		
		$( $crate::make_uname_data! {
			$($unk)*
		})?
	};
	{} => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __next_macrocode_for_feature {
	[ $($code:tt)* ] => { $($code)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_uname_structdata {
	[
		$(#[$($cfg_all:tt)*])*
		@make [$($addition_pub: tt)*] [ $(#for_new($addition_new:tt): )? $enum_data: ident { $( $enum_ident: ident : $enum_ty: path ),* $(,)? } ] 
	] => {
		$(#[$($cfg_all)*])*
		$($addition_pub)* struct $enum_data {
			$( pub (crate) $enum_ident : $enum_ty ),*
		}
		
		impl $enum_data {
			/// Creating a `Uname` data with a user data set.
			#[inline]
			pub const $($addition_new)? fn new(
				$( $enum_ident : $enum_ty ),*
			) -> Self {
				Self {
					$( $enum_ident),*
				}
			}
		}
	};
	[ @get_type [ $(#for_new($addition_new:tt): )?  $enum_data: ident { $($unk:tt)* } ] ] => { $enum_data };
	
	[
		$(#[$($cfg_all:tt)*])*
		@make [$($addition_pub: tt)*][ $(#for_new($addition_new:tt): )?  $enum_data:ty ]
	] => {};
	[ @get_type [ $(#for_new($addition_new:tt): )?  $($enum_data:tt)* ] ] => { $($enum_data)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_trait_for_uname {
	[
		@[$($enum_name:tt)*]:
		impl GetUname <$as_uname:ty> for #$for:tt {
			#ref($ref_name: tt) => $asname:ident <$asname_ty:ty>;
		}
		
		$($unk:tt)*
	] => {
		$crate::__make_trait_for_uname! {
			@[$($enum_name)*]:
			impl GetUname <$as_uname> for #$for {
				#[inline(always)] sysname	|data| (<$ref_name as $asname<$asname_ty>>::get_sysname(data)),
				#[inline(always)] nodename	|data| (<$ref_name as $asname<$asname_ty>>::get_nodename(data)),
				#[inline(always)] release	|data| (<$ref_name as $asname<$asname_ty>>::get_release(data)),
				#[inline(always)] version	|data| (<$ref_name as $asname<$asname_ty>>::get_version(data)),
				#[inline(always)] machine	|data| (<$ref_name as $asname<$asname_ty>>::get_machine(data)),
				#[inline(always)] domainname	|data| (<$ref_name as $asname<$asname_ty>>::get_domainname(data)),
			}
			
			$($unk)*
		}
	};
	[
		@[$($enum_name:tt)*]:
		impl AsPtrUname <$as_uname:ty> for #$for:tt {
			#ref($ref_name: tt) => $asname:ident <$asname_ty:ty>;
		}
		
		$($unk:tt)*
	] => {
		$crate::__make_trait_for_uname! {
			@[$($enum_name)*]:
			impl AsPtrUname <$as_uname> for #$for {
				#[inline(always)] sysname	|data| (<$ref_name as $asname<$asname_ty>>::as_ptr_sysname(data)),
				#[inline(always)] nodename	|data| (<$ref_name as $asname<$asname_ty>>::as_ptr_nodename(data)),
				#[inline(always)] release	|data| (<$ref_name as $asname<$asname_ty>>::as_ptr_release(data)),
				#[inline(always)] version	|data| (<$ref_name as $asname<$asname_ty>>::as_ptr_version(data)),
				#[inline(always)] machine	|data| (<$ref_name as $asname<$asname_ty>>::as_ptr_machine(data)),
				#[inline(always)] domainname	|data| (<$ref_name as $asname<$asname_ty>>::as_ptr_domainname(data)),
			}
			
			$($unk)*
		}
	};
	
	[ // AsPtrUname
		@[$($enum_name:tt)*]:
		
		impl AsPtrUname<$as_uname:ty> for #self {
			$(#[ $($sysname_addition: tt)* ])*
			sysname	|$sysname_data: ident| $(($sysname_expr:expr))? $($sysname:block)?, // EXPR || BLOCK
			
			$(#[ $($nodename_addition: tt)* ])*
			nodename	|$nodename_data: ident| $(($nodename_expr:expr))? $($nodename:block)?, // EXPR || BLOCK
			
			$(#[ $($release_addition: tt)* ])*
			release	|$release_data: ident| $(($release_expr:expr))? $($release:block)?, // EXPR || BLOCK
			
			$(#[ $($version_addition: tt)* ])*
			version	|$version_data: ident| $(($version_expr:expr))? $($version:block)?, // EXPR || BLOCK
			
			$(#[ $($machine_addition: tt)* ])*
			machine	|$machine_data: ident| $(($machine_expr:expr))? $($machine:block)?, // EXPR || BLOCK
			
			$(#[ $($domainname_addition: tt)* ])*
			domainname|$domainname_data: ident| $(($domainname_expr:expr))? $($domainname:block)? $(,)? // EXPR || BLOCK
		}
		$($unk:tt)*
	] => {
		impl $crate::AsPtrUname<$as_uname> for $($enum_name)* {
			type Data = <$($enum_name)* as $crate::UnameBeh>::Data;
			
			$(#[ $($sysname_addition)* ])*
			fn as_ptr_sysname($sysname_data: &Self::Data) -> *const $as_uname {
				$($sysname_expr)?
				$($sysname)?
			}
			
			$(#[ $($nodename_addition)* ])*
			fn as_ptr_nodename($nodename_data: &Self::Data) -> *const $as_uname {
				$($nodename_expr)?
				$($nodename)?
			}
			
			$(#[ $($release_addition)* ])*
			fn as_ptr_release($release_data: &Self::Data) -> *const $as_uname {
				$($release_expr)?
				$($release)?
			}
			
			$(#[ $($version_addition)* ])*
			fn as_ptr_version($version_data: &Self::Data) -> *const $as_uname {
				$($version_expr)?
				$($version)?
			}
			
			$(#[ $($machine_addition)* ])*
			fn as_ptr_machine($machine_data: &Self::Data) -> *const $as_uname {
				$($machine_expr)?
				$($machine)?
			}
			
			$(#[ $($machine_addition)* ])*
			fn as_ptr_domainname($domainname_data: &Self::Data) -> *const $as_uname {
				$($domainname_expr)?
				$($domainname)?
			}
		}
		$crate::__make_trait_for_uname! {
			@[$($enum_name)*]:
			
			$($unk)*
		}
	};
	
	[ // GetUname
		@[$($enum_name:tt)*]:
		
		impl GetUname<$as_uname:ty> for #self {
			$(#[ $($sysname_addition: tt)* ])*
			sysname	|$sysname_data: ident| $(($sysname_expr:expr))? $($sysname:block)?, // EXPR || BLOCK
			
			$(#[ $($nodename_addition: tt)* ])*
			nodename	|$nodename_data: ident| $(($nodename_expr:expr))? $($nodename:block)?, // EXPR || BLOCK
			
			$(#[ $($release_addition: tt)* ])*
			release	|$release_data: ident| $(($release_expr:expr))? $($release:block)?, // EXPR || BLOCK
			
			$(#[ $($version_addition: tt)* ])*
			version	|$version_data: ident| $(($version_expr:expr))? $($version:block)?, // EXPR || BLOCK
			
			$(#[ $($machine_addition: tt)* ])*
			machine	|$machine_data: ident| $(($machine_expr:expr))? $($machine:block)?, // EXPR || BLOCK
			
			$(#[ $($domainname_addition: tt)* ])*
			domainname|$domainname_data: ident| $(($domainname_expr:expr))? $($domainname:block)? $(,)? // EXPR || BLOCK
		}
		$($unk:tt)*
	] => {
		impl $crate::GetUname<$as_uname> for $($enum_name)* {
			type Data = <$($enum_name)* as $crate::UnameBeh>::Data;
			
			$(#[ $($sysname_addition)* ])*
			fn get_sysname($sysname_data: &Self::Data) -> &$as_uname {
				$($sysname_expr)?
				$($sysname)?
			}
			
			$(#[ $($nodename_addition)* ])*
			fn get_nodename($nodename_data: &Self::Data) -> &$as_uname {
				$($nodename_expr)?
				$($nodename)?
			}
			
			$(#[ $($release_addition)* ])*
			fn get_release($release_data: &Self::Data) -> &$as_uname {
				$($release_expr)?
				$($release)?
			}
			
			$(#[ $($version_addition)* ])*
			fn get_version($version_data: &Self::Data) -> &$as_uname {
				$($version_expr)?
				$($version)?
			}
			
			$(#[ $($machine_addition)* ])*
			fn get_machine($machine_data: &Self::Data) -> &$as_uname {
				$($machine_expr)?
				$($machine)?
			}
			
			$(#[ $($machine_addition)* ])*
			fn get_domainname($domainname_data: &Self::Data) -> &$as_uname {
				$($domainname_expr)?
				$($domainname)?
			}
		}
		$crate::__make_trait_for_uname! {
			@[$($enum_name)*]:
			
			$($unk)*
		}
	};
	[
		@[$($enum_name:tt)*]:
	] => {}
}
