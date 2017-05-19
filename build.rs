extern crate pnet_macros;
extern crate syntex;
extern crate glob;

use glob::glob;
use std::env;
use std::path::Path;


fn find_files( pattern: &str, sep: &str ) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	for entry in glob( pattern ).expect("Failed to read glob pattern") {
		match entry {
			Ok(path) => {
                let temp = path.display().to_string();
                let elms = temp.split( sep ).collect::<Vec<_>>();
				let mut name = String::new();
				if elms.len()>0 {
					name += elms[..elms.len()-1].join(sep).as_str();
				}
                println!(" -- {:?} => {:?}", path.display().to_string(), name);
                ret.push( vec![name, path.file_name().to_string()] )
			},
			Err(_) => {},
		}
	}
    ret
}

fn main() {
    for f in find_files( "./**/*.rs.in", ".in") {
		let mut registry = syntex::Registry::new();
		pnet_macros::register(&mut registry);
		let src_string = format!("{}.in", f.as_str());
		let dst_string = format!("{}",    f.as_str());
		let src = Path::new( src_string.as_str() );
		let dst = Path::new( &env::var_os("OUT_DIR").unwrap().join(dst_string.as_str() );
		registry.expand("", &src, &dst).unwrap();
    }
    /*
    let src  = Path::new("src/packet/my_protocol.rs.in");
    let dst  = Path::new(&env::var_os("OUT_DIR").unwrap()).join("my_protocol.rs");
    registry.expand("", &src, &dst).unwrap();
    */
    
}
