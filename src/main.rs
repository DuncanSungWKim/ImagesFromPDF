#![ allow( non_snake_case ) ]

use std::{
    env
,   path::Path, fs, io::prelude::*
};

use pdf::{
    file::FileOptions, error::Result, object::*
,   enc::StreamFilter
};



fn main() {
    let fileName = GetPDFFileName() ;
    let mut imageCollector = ImageCollector::newWithSavingFolder( fileName.as_str() ) ;
    let file = FileOptions::cached().open( &fileName ).unwrap() ;
    for page in file.pages() {
        imageCollector.ClipImagesFrom( page.unwrap(), &file ) ;
    }
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
,   pageCount : u16
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
        ,   pageCount : 0
        }
    }


    fn ClipImagesFrom( &mut self
    ,   page : PageRc
    ,   resolver : &impl Resolve
    )-> Result< () >
    {
        self.pageCount += 1 ;
        let mut imageCount : u16 = 0 ;
        for ( _name, &r ) in page.resources()?.xobjects.iter() {
            let XObject::Image( ref img ) = *resolver.get( r )? else {
                continue ;
            };
            let ( data, filter ) = img.raw_image_data( resolver )? ;
            let ext = match filter {
                Some(StreamFilter::DCTDecode(_)) => "jpg"
            ,   Some(StreamFilter::JBIG2Decode)  => "jbig2"
            ,   Some(StreamFilter::JPXDecode)    => "jp2k"
            ,   _=> continue
            };
            imageCount += 1 ;
            let fileName = format!( "{:04}-{:02}.{ext}", self.pageCount, imageCount ) ;
            let filePath = Path::new( &self.folderName ).join( fileName ) ;
            fs::write( filePath, data )? ;
        }
        Ok( () )
    }
}
