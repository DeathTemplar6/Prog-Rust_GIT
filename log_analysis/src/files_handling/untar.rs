use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

///Uncompressed tar.gz achive passed to it. It need tar path to work
pub fn untar(args: &[String]) -> Result<(), &str> {
	// Check that an argument was given
	if args.len() < 2 {
            return Err("not enough arguments");
    }
	
    let path = args[1].clone();
	
	let file = File::open(path);
	//check that the file exist / can be opened
    let file = match file {
        Ok(file) => file,
        Err(_) => return Err("Problem opening the file, file probably doesn't exist"),
    };
	//Uncompressed command from crate
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
	
	let arch_unpack = archive.unpack("./log");
	//check that it can be uncompressed, so that the doc is an archive tar.gz
    match arch_unpack {
        Ok(_) => (),
        Err(_error) => return Err("Problem unpacking the file, it's probably not a tar.gz"),
    };

    Ok(())
}
