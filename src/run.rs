use crate::core;


pub fn start_bot() {
    let a = core::get_me().unwrap();
    println!("{:#?}", a);
}