//! Bindings to Flint 2.5.2. Mostly generated with rust-bindgen, modified for readability.
use crate::num::mpz::flint_mpz_struct;

#[allow(non_camel_case_types)]
pub type fmpz = ::std::os::raw::c_long;

extern "C" {
    #[link_name = "\u{1}_fmpz_get_mpz"]
    pub fn fmpz_get_mpz(x: *mut flint_mpz_struct, f: *mut fmpz);
}
extern "C" {
    #[link_name = "\u{1}_fmpz_set_mpz"]
    pub fn fmpz_set_mpz(f: *mut fmpz, x: *mut flint_mpz_struct);
}
extern "C" {
    #[link_name = "\u{1}_fmpz_xgcd_partial"]
    pub fn fmpz_xgcd_partial(
        co2: *mut fmpz,
        co1: *mut fmpz,
        r2: *mut fmpz,
        r1: *mut fmpz,
        L: *mut fmpz,
    );
}

#[test]
fn bindgen_test_layout_flint_mpz_struct() {
    assert_eq!(
        ::std::mem::size_of::<flint_mpz_struct>(),
        16usize,
        concat!("Size of: ", stringify!(flint_mpz_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<flint_mpz_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(flint_mpz_struct))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flint_mpz_struct>())).mp_alloc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(flint_mpz_struct),
            "::",
            stringify!(_mp_alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flint_mpz_struct>())).mp_size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(flint_mpz_struct),
            "::",
            stringify!(_mp_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<flint_mpz_struct>())).mp_d as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(flint_mpz_struct),
            "::",
            stringify!(_mp_d)
        )
    );
}