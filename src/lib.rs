#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn initialize_and_finalize_margo() {
        unsafe {
            let address: CString = CString::new("na+sm").unwrap();
            let mid: margo_instance_id = margo_init(address.as_ptr(), MARGO_SERVER_MODE as i32, 0, 0);
            margo_finalize(mid);
        }
    }

}
