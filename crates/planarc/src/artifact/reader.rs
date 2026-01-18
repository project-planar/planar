use std::io;
use thiserror::Error;
use xxhash_rust::xxh64::xxh64;
use rkyv::Archived;

use crate::artifact::model::ArchivedProgram;

use super::header::{Header, MAGIC, VERSION};
use super::model::Program;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid magic bytes. Expected 'PDLA', got {0:?}")]
    InvalidMagic([u8; 4]),
    #[error("Version mismatch: file v{file}, runtime v{runtime}")]
    VersionMismatch { file: u32, runtime: u32 },
    #[error("Checksum mismatch: expected {expected:x}, got {calculated:x}")]
    ChecksumMismatch { expected: u64, calculated: u64 },
    #[error("File too short")]
    Truncated,
}

pub struct LoadedProgram<'a> {
    pub archived: &'a Archived<Program>,
}

pub fn load_program<'a>(
    data: &'a [u8],
) -> Result<LoadedProgram<'a>, LoadError> {
    
    if data.len() < 16 {
        return Err(LoadError::Truncated);
    }

    let (header_bytes, payload) = data.split_at(16);
    let header = Header::from_bytes(header_bytes.try_into().unwrap());

    
    if &header.magic != MAGIC {
        return Err(LoadError::InvalidMagic(header.magic));
    }

    
    if header.version != VERSION {
        return Err(LoadError::VersionMismatch {
            file: header.version,
            runtime: VERSION,
        });
    }

    let calculated = xxh64(payload, 0);
    if calculated != header.checksum {
        return Err(LoadError::ChecksumMismatch {
            expected: header.checksum,
            calculated,
        });
    }

    let archived = unsafe { rkyv::access_unchecked::<ArchivedProgram>(payload) };

    Ok(LoadedProgram { archived })
}