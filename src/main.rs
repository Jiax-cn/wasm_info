use std::{
    env,
	fs::read,
	path::Path,
};

use wasm_info::{
    common::Module,
    custom::get_name_map,
};

fn main(){
    let args = env::args().collect::<Vec<_>>();
	if args.len() != 2 {
		println!("Usage: {} input.wasm", args[0]);
		return;
	}

    let path = Path::new(&args[1]);
    let bytes = read(path).unwrap();

    // Loading module
    let module = Module::new(bytes.as_ref()).unwrap();

    if let Some(name_map) = get_name_map(&module){
        for naming in name_map.into_iter() {
            let naming = naming.unwrap();
            println!("{} {}", naming.index, naming.name);
        }
    }
    return;
}