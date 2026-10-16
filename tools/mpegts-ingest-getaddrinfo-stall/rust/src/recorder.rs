// pcap writer (nanosecond timestamps, Ethernet link type).

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub fn write_pcap(path: &Path, frames: &[(u64, Vec<u8>)]) -> io::Result<u64> {
    let mut w = BufWriter::new(File::create(path)?);
    w.write_all(&0xa1b2_3c4du32.to_le_bytes())?; // nanosecond-resolution magic
    w.write_all(&2u16.to_le_bytes())?;
    w.write_all(&4u16.to_le_bytes())?;
    w.write_all(&0i32.to_le_bytes())?;
    w.write_all(&0u32.to_le_bytes())?;
    w.write_all(&262_144u32.to_le_bytes())?;
    w.write_all(&1u32.to_le_bytes())?; // LINKTYPE_ETHERNET
    let mut bytes = 24u64;
    for (ts, f) in frames {
        w.write_all(&((ts / 1_000_000_000) as u32).to_le_bytes())?;
        w.write_all(&((ts % 1_000_000_000) as u32).to_le_bytes())?;
        w.write_all(&(f.len() as u32).to_le_bytes())?;
        w.write_all(&(f.len() as u32).to_le_bytes())?;
        w.write_all(f)?;
        bytes += 16 + f.len() as u64;
    }
    w.flush()?;
    Ok(bytes)
}
