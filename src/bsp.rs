use std::{io::Write, u32};
use color_eyre::eyre::Result;


#[repr(usize)]
enum Lump {
    Entities    = 0,
    Textures    = 1,
    Planes      = 2,
    Nodes       = 3,
    Leafs       = 4,
    Leaffaces   = 5,
    Leafbrushes = 6,
    Models      = 7,
    Brushes     = 8,
    Brushsides  = 9,
    Vertices    = 10,
    Meshverts   = 11,
    Effects     = 12,
    Faces       = 13,
    Lightmaps   = 14,
    Lightvols   = 15,
    Visdata     = 16,
}

pub trait BinWriter {
    fn writeout<W>(&self, wtr: &mut W) -> Result<()> where W: Write;
}

struct Direntry {
    off: u32,
    len: u32,
}

impl BinWriter for Direntry {
    fn writeout<W>(&self, wtr: &mut W) -> Result<()>
    where W: Write {
        wtr.write_all(&u32::to_le_bytes(self.off))?;
        wtr.write_all(&u32::to_le_bytes(self.len))?;

        Ok(())
    }
}

struct Header {
    version: u32,
    direntries: [Direntry; 17],
}

impl BinWriter for Header {
    fn writeout<W>(&self, wtr: &mut W) -> Result<()>
    where W: Write {
        wtr.write_all(b"IBSP")?;
        wtr.write_all(&u32::to_le_bytes(self.version))?;
        for entry in &self.direntries {
            entry.writeout(wtr)?;
        }

        Ok(())
    }
}