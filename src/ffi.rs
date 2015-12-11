use std::mem;
use std::slice;
use libc::{c_char, uint8_t, uint32_t, int32_t, c_void};

#[repr(C)]
pub struct Keypair;

#[link(name = "mysnark")]
extern "C" {
    pub fn mysnark_init_public_params();
    fn gen_keypair(n: uint32_t, h: *mut c_void, cb: extern fn(*mut c_void, *const c_char, int32_t, *const c_char, int32_t));
    fn load_keypair(pk_s: *const c_char, pk_l: int32_t, vk_s: *const c_char, vk_l: int32_t)
        -> *const Keypair;
    fn gen_proof(keypair: *const Keypair, n: uint32_t, puzzle: *const uint8_t, solution: *const uint8_t,
                 key: *const uint8_t, h_of_key: *const uint8_t) -> bool;
}

extern "C" fn handle_keypair_callback(cb: *mut c_void, pk_s: *const c_char, pk_l: int32_t, vk_s: *const c_char, vk_l: int32_t)
{
    unsafe {
        let pk: &[i8] = mem::transmute(slice::from_raw_parts(pk_s, pk_l as usize));
        let vk: &[i8] = mem::transmute(slice::from_raw_parts(vk_s, vk_l as usize));

        let closure: &mut &mut for<'a> FnMut(&'a [i8], &'a [i8]) = mem::transmute(cb);

        closure(pk, vk);
    }
}

pub fn generate_keypair<F: for<'a> FnMut(&'a [i8], &'a [i8])>(num: u32, mut f: F) {
    let mut cb: &mut for<'a> FnMut(&'a [i8], &'a [i8]) = &mut f;

    unsafe {
        gen_keypair(num, (&mut cb) as *mut _ as *mut c_void, handle_keypair_callback);
    }
}

impl Keypair {
    pub fn new(pk: &[i8], vk: &[i8]) -> *const Keypair {
        unsafe {
            load_keypair(&pk[0], pk.len() as i32, &vk[0], vk.len() as i32)
        }
    }
}

pub fn prove(pk: *const Keypair, n: usize, puzzle: &[u8], solution: &[u8], key: &[u8], h_of_key: &[u8]) -> bool {
    assert_eq!(puzzle.len(), n*n*n*n);
    assert_eq!(solution.len(), n*n*n*n);

    unsafe {
        gen_proof(pk, n as u32, &puzzle[0], &solution[0], &key[0], &h_of_key[0])
    }
}