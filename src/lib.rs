uniffi::include_scaffolding!("mathrust");

#[no_mangle]
pub fn add(a: u32, b: u32) -> u32 {
    a+b
}

#[no_mangle]
pub fn hello() -> String {
	"Hello world".to_string()
}

#[no_mangle]
pub fn hello2() {
	println!("Hi from rust");
}