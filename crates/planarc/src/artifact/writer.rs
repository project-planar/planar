use std::io::{self, Write};
use rkyv::rancor::Error;
use xxhash_rust::xxh64::xxh64;

use super::header::Header;
use super::model::Program;

pub fn write_program(program: &Program, writer: &mut impl Write) -> io::Result<()> {
    
    let bytes = rkyv::to_bytes::<Error>(program)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let checksum = xxh64(&bytes, 0);

    let header = Header::new(checksum);

    writer.write_all(&header.as_bytes())?;

    writer.write_all(&bytes)?;

    Ok(())
}