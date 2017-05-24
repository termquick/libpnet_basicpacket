fn main() {
    extern crate pnet_macros;
    extern crate syntex;
    extern crate glob;
    
    //use pnet_macros;
    //use syntex;
    //use glob::glob;
    use std::env;
    use std::path::Path;
    
    // globbing for files to pre-process:
    let pattern = "./**/.rs.in";
    for entry in glob::glob( pattern ).expect("Failed to read glob pattern") {
	    match entry {
	        Ok(path) => {
                println!(" CMP-- {}", path.display() );
                //src: Path::new() = /full/path/file.rs.in
                let src     = Path::new( path.to_str().unwrap() );
                //src -> dst: Path::new() = OUT_DIR/file.rs
                let out_dir = env::var_os( "OUT_DIR" ).unwrap();
                let file    = Path::new( path.file_stem().unwrap() );
                let dst     = Path::new( &out_dir ).join(file);
                let mut registry = syntex::Registry::new();
                pnet_macros::register(&mut registry);
                registry.expand("", &src, &dst).unwrap();
            },
	    	Err(_) => {},
        }
    }
}
