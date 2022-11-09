#![feature(c_unwind)]

use flate2::{bufread::GzDecoder, write::GzEncoder, Compression};
use log::trace;
use rhsp3_common::*;
use rhsp3_plugsdk::{hsp_export, ObjectStore, Var, VarBuffer};
use std::{
    cell::RefCell,
    ffi::c_int,
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

struct ZLibData {
    streams: ObjectStore<RefCell<ZLibHandle>>,
}
impl HspExtData for ZLibData {
    fn init() -> Result<Self> {
        Ok(ZLibData { streams: ObjectStore::new()? })
    }
}

enum ZLibStream {
    Read(GzDecoder<BufReader<File>>),
    Write(GzEncoder<File>),
}
struct ZLibHandle {
    stream: ZLibStream,
}
impl ZLibHandle {
    fn open_read(path: impl AsRef<Path>) -> Result<Self> {
        Ok(ZLibHandle {
            stream: ZLibStream::Read(GzDecoder::new(BufReader::new(File::open(path)?))),
        })
    }
    fn open_write(path: impl AsRef<Path>) -> Result<Self> {
        Ok(ZLibHandle {
            stream: ZLibStream::Write(GzEncoder::new(File::create(path)?, Compression::best())),
        })
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<usize> {
        match &mut self.stream {
            ZLibStream::Read(stream) => {
                let mut read = 0;
                while read != buffer.len() {
                    let call_read = stream.read(&mut buffer[read..])?;
                    if call_read == 0 {
                        break;
                    }
                    read += call_read;
                }
                Ok(read)
            }
            ZLibStream::Write(_) => bail_lit!("Cannot read from a write stream."),
        }
    }
    fn write_bytes(&mut self, buffer: &[u8]) -> Result<usize> {
        match &mut self.stream {
            ZLibStream::Write(stream) => match stream.write_all(buffer) {
                Ok(()) => Ok(buffer.len()),
                Err(_) => Ok(0),
            },
            ZLibStream::Read(_) => bail_lit!("Cannot write into a read stream."),
        }
    }

    fn finish(&mut self) -> Result<()> {
        match &mut self.stream {
            ZLibStream::Write(stream) => stream.try_finish()?,
            ZLibStream::Read(_) => {}
        }
        Ok(())
    }
}

const Z_READ: i32 = 0;
const Z_WRITE: i32 = 1;

#[hsp_export]
fn zlib_open(
    #[ext_data] ctx: &mut ZLibData,
    p_handle: &mut impl Var<c_int>,
    name: &str,
    mode: i32,
) -> Result<()> {
    let handle = match mode {
        Z_READ => ZLibHandle::open_read(Path::new(name))?,
        Z_WRITE => ZLibHandle::open_write(Path::new(name))?,
        _ => bail_code!(InvalidParameter),
    };

    let idx = ctx.streams.alloc(RefCell::new(handle))?;
    p_handle.set(idx)?;
    trace!("zlib_open({p_handle:?}, {name:?}, {mode}) = idx");

    Ok(())
}

#[hsp_export]
fn zlib_read(
    #[ext_data] ctx: &mut ZLibData,
    data: &mut impl VarBuffer,
    p_handle: c_int,
    size: usize,
) -> Result<i32> {
    let handle = ctx.streams.get(p_handle).unwrap();
    ensure_lit!(size <= data.len(), "Given buffer is not large enough!");
    let result = handle.borrow_mut().read_bytes(&mut *data)?;
    Ok(-(result as i32))
}

#[hsp_export]
fn zlib_write(
    #[ext_data] ctx: &mut ZLibData,
    data: &mut impl VarBuffer,
    p_handle: c_int,
    size: usize,
) -> Result<i32> {
    let handle = ctx.streams.get(p_handle).unwrap();
    ensure_lit!(size <= data.len(), "Given buffer is not large enough!");
    let result = handle.borrow_mut().write_bytes(&*data)?;
    Ok(-(result as i32))
}

#[hsp_export]
fn zlib_free(#[ext_data] ctx: &mut ZLibData, p_handle: c_int) -> Result<()> {
    trace!("zlib_free({p_handle})");
    ctx.streams.free(p_handle)?.borrow_mut().finish()?;
    Ok(())
}

rhsp3_plugsdk::hpi_root!(pub ZlibHpi);
