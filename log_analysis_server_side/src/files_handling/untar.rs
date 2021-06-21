use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

///Uncompressed tar.gz achive passed to it. It need tar path to work
pub fn untar(file: String) -> Result<(), &'static str> {
	// Check that an argument was given
	if file.is_empty() {
            return Err("tar : file_name is empty");
    }
	
    let path = file.clone();
	
	let file = File::open(path);
	//check that the file exist / can be opened
    let file = match file {
        Ok(file) => file,
        Err(_) => return Err("Problem opening the archive"),
    };
	//Uncompressed command from crate
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
	
	let arch_unpack = archive.unpack("./log");
	//check that it can be uncompressed, so that the doc is an archive tar.gz
    match arch_unpack {
        Ok(_) => (),
        Err(_) => return Err("Problem unpacking the file"),
    };

    Ok(())
}
