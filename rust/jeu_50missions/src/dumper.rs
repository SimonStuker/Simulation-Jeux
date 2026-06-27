use std::{fs::File, io::BufWriter, path::PathBuf};

use crate::simulation::SimulationResult;

pub fn write_results(results: &[SimulationResult], output_path: &PathBuf) -> std::io::Result<()> {
    let file = File::create(output_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, results)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
