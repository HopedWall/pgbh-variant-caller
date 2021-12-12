use std::io;
use std::path::PathBuf;

use csv;
use handlegraph::handle::Edge;

use crate::bubbles::Bubble;

pub(crate) fn write_edges_to_file(
    graph_id: &u64,
    edges: &Vec<Edge>,
    path: &PathBuf,
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer.write_record(&["Graph id", "Src", "Dest"]).unwrap();
    for edge in edges {
        writer
            .write_record(&[
                graph_id.to_string(),
                edge.0.as_integer().to_string(),
                edge.1.as_integer().to_string(),
            ])
            .unwrap();
    }
    writer.flush().unwrap();

    Ok(())
}

pub(crate) fn write_bubbles_to_file(
    graph_id: &u64,
    bubbles: &Vec<Bubble>,
    path: &PathBuf,
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["Graph id", "Bubble Start", "Bubble End"])
        .unwrap();
    for bubble in bubbles {
        writer
            .write_record(&[
                graph_id.to_string(),
                bubble.start.to_string(),
                bubble.end.to_string(),
            ])
            .unwrap();
    }
    writer.flush().unwrap();

    Ok(())
}

pub(crate) fn write_all_edges_to_file(
    edges_with_id: &Vec<(u64, Edge)>,
    path: &PathBuf,
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer.write_record(&["Graph id", "Src", "Dest"]).unwrap();
    for (graph_id, edge) in edges_with_id {
        writer
            .write_record(&[
                graph_id.to_string(),
                edge.0.as_integer().to_string(),
                edge.1.as_integer().to_string(),
            ])
            .unwrap();
    }
    writer.flush().unwrap();

    Ok(())
}

pub(crate) fn write_all_bubbles_to_file(
    bubbles_with_id: &Vec<(u64, Bubble)>,
    path: &PathBuf,
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(&["Graph id", "Bubble Start", "Bubble End"])
        .unwrap();
    for (graph_id, bubble) in bubbles_with_id {
        writer
            .write_record(&[
                graph_id.to_string(),
                bubble.start.to_string(),
                bubble.end.to_string(),
            ])
            .unwrap();
    }
    writer.flush().unwrap();

    Ok(())
}
