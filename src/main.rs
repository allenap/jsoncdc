extern crate elephantpump_sys;

fn main() {
    unsafe {
        println!("Hello World! {}", elephantpump_sys::wal_level);
    }
}

