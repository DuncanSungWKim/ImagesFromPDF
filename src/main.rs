use std::{
    env
,   path::Path, fs, io::prelude::*
};



fn main() {
    let fileName = GetPDFFileName() ;
    let mut imageCollector = ImageCollector::newWithSavingFolder( fileName.as_str() ) ;
}



fn GetPDFFileName()
-> String
{
    let arg1 = env::args().nth( 1 ) ;
    arg1.expect( "PDF file as first argument" )
}



struct ImageCollector
{
    folderName : String
}


impl ImageCollector
{
    fn newWithSavingFolder(
        fileName : &str
    )-> Self
    {
        let folderName = Path::new( fileName ).file_stem().unwrap() ;
        let folder = Path::new( folderName ) ;
        if folder.is_dir() {
            fs::remove_dir_all( folder ) ;
        }
        fs::create_dir( folder ) ;
        Self {
            folderName : folderName.to_str().unwrap().to_owned()
        }
    }
}
