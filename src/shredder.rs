use constants;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::io::{BufWriter, Error, SeekFrom};
use std::path::{Path, PathBuf};

pub struct Shred {
    num_pass: usize,
    files_shreded: usize,
    buffer: Vec<u8>,
}

//pub struct ShredOptions

pub enum ShredType {
    Ones, // 255
    Random,
    Zeroes, // 0
}

impl Shred {
    pub fn new() -> Shred {
        Shred {
            num_pass: constants::NUM_PASSES,
            files_shreded: 0usize,
            buffer: Shred::create_buffer(constants::SHRED_TYPE),
        }
    }
    
    pub fn shred(&mut self, path: &Path) -> Result<(), Error> {
        let metadata = try!(fs::metadata(path));
        if metadata.is_dir() {        
            try!(self.shred_dir(path));
        } else {
            try!(self.shred_file(path));
        }
        Ok(())
    }

fn shred_dir(&mut self, path: &Path) -> Result<(), Error> {
        for child in try!(fs::read_dir(path)) {
            let child = try!(child);
            let child_type = try!(child.file_type());
            if child_type.is_dir() {
                try!(self.shred_dir(&child.path()));
           } else if child_type.is_symlink() {
                try!(fs::remove_file(&child.path()));
            } else {
                try!(self.shred_file(&child.path()));
            }
        }
        try!(fs::remove_dir(path));
        Ok(())
    }

    fn shred_file(&mut self, path: &Path) -> Result<(), Error> {  
        try!(self.shred_write(&path));        
        let file: PathBuf = try!(self.rename(&path));
        try!(fs::remove_file(file));
        Ok(())
    }

    fn create_buffer(shred_type: ShredType) -> Vec<u8> {
        let buffer: Vec<u8>;
        match shred_type {
            ShredType::Ones => buffer = vec![255; constants::BUFFER_SIZE],
            ShredType::Zeroes => buffer = vec![0; constants::BUFFER_SIZE],
            ShredType::Random => unimplemented!(), //buffer = random_buffer(),
        } 
        buffer

    }   



    fn shred_write(&self, path: &Path) -> Result<(), Error> {
        let file = try!(OpenOptions::new()
            .read(false)
            .write(true)
            .create(false)
            .open(path));       
        let file_size = try!(file.metadata()).len();
        let mut buf_write = BufWriter::new(file); 
        let buffer_slice = &self.buffer[..];  
        let buffer_slice_len = self.buffer.len() as u64;

        for _ in 0..self.num_pass {
            let mut seek = 0u64;
            while file_size > seek {
                try!(buf_write.seek(SeekFrom::Start(seek)));
                try!(buf_write.write(buffer_slice));
                seek += buffer_slice_len;                
            }
        }
        Ok(())
    }

    fn rename(&mut self, path: &Path) -> Result<PathBuf, Error> {
        let mut pathbuf = PathBuf::from(path);
        self.files_shreded += 1;
        let filename = "___shredded".to_string() + "_" + &self.files_shreded.to_string();
        pathbuf.set_file_name(filename);
        pathbuf.set_extension("file");
        try!(fs::rename(path, &pathbuf));
        Ok(pathbuf)
    }
}