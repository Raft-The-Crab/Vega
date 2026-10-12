//! Tile data serialization
//!
//! Provides serialization of tile pixel data to the .vega file format.

use std::io::{BufReader, BufWriter, Write};
use vega_core::tiles::{Tile, TileCoords, TileStorage};
use crate::Result;
use crate::vega::FormatError;

/// Tile data serializer.
pub struct TileSerializer;

impl TileSerializer {
    /// Serializes all tiles to the ZIP archive.
    pub fn serialize_tiles<W: Write>(
        tile_storage: &TileStorage,
        zip: &mut zip::ZipWriter<W>,
    ) -> Result<()> {
        let tile_size = tile_storage.tile_size();

        // Iterate over all allocated tiles
        for (coords, handle) in tile_storage.iter() {
            if let Some(tile) = tile_storage.get_tile(handle) {
                let tile_data = tile.data();
                let tile_x = coords.x;
                let tile_y = coords.y;

                // Create path: tiles/{tile_x}_{tile_y}.tile
                let path = format!("tiles/{}_{}.tile", tile_x, tile_y);
                
                zip.start_file(&path, zip::write::FileOptions::default())?;
                zip.write_all(tile_data)?;
            }
        }

        Ok(())
    }

    /// Deserializes tiles from the ZIP archive.
    pub fn deserialize_tiles<R: std::io::Read + std::io::Seek>(
        zip: &mut zip::ZipArchive<std::io::BufReader<R>>,
        tile_storage: &mut TileStorage,
    ) -> Result<()> {
        let tile_size = tile_storage.tile_size();

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            let name = file.name();

            if name.starts_with("tiles/") && name.ends_with(".tile") {
                // Extract tile coordinates from path: "tiles/{x}_{y}.tile"
                let parts: Vec<&str> = name.split('/').collect();
                if parts.len() >= 2 {
                    let filename = parts[1].trim_end_matches(".tile");
                    let coords_parts: Vec<&str> = filename.split('_').collect();

                    if coords_parts.len() == 2 {
                        if let (Ok(x), Ok(y)) = (
                            coords_parts[0].parse::<u32>(),
                            coords_parts[1].parse::<u32>(),
                        ) {
                            let coords = TileCoords::new(x, y);
                            let handle = tile_storage.get_or_create(coords)?;

                            if let Some(tile) = tile_storage.get_tile_mut(handle) {
                                let mut tile_data = Vec::new();
                                BufReader::new(file).read_to_end(&mut tile_data)?;

                                // Copy data to tile
                                let tile_pixels = tile.data_mut();
                                tile_pixels.copy_from_slice(&tile_data);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Counts tiles in the ZIP archive.
    pub fn count_tiles<R: std::io::Read + std::io::Seek>(
        zip: &mut zip::ZipArchive<std::io::BufReader<R>>,
    ) -> Result<usize> {
        let mut count = 0;

        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            let name = file.name();

            if name.starts_with("tiles/") && name.ends_with(".tile") {
                count += 1;
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vega_core::RGBA8;
    use tempfile::TempDir;

    #[test]
    fn test_tile_serialization() {
        let temp_dir = TempDir::new().unwrap();
        let zip_path = temp_dir.path().join("test.zip");

        let file = std::fs::File::create(&zip_path).unwrap();
        let writer = BufWriter::new(file);
        let mut zip = zip::ZipWriter::new(writer);

        let mut tile_storage = TileStorage::new(100, 100, 64).unwrap();
        let coords = TileCoords::new(0, 0);
        let handle = tile_storage.get_or_create(coords).unwrap();

        if let Some(tile) = tile_storage.get_tile_mut(handle) {
            tile.fill(RGBA8::RED);
        }

        TileSerializer::serialize_tiles(&tile_storage, &mut zip).unwrap();
        zip.finish().unwrap();
    }
}
