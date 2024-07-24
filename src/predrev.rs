// Copyright (C) 2023  The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

use anyhow::{Context, Result};
use bitvec::prelude::*;
use clap::Parser;
use dsi_progress_logger::ProgressLogger;
use log::{debug, info};
use std::collections::VecDeque;
use std::path::PathBuf;
use swh_graph::graph::*;
use swh_graph::NodeType;
use swh_graph::java_compat::mph::gov::GOVMPH;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    graph: PathBuf,
    #[arg(short, long)]
    swhid: String,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    // Setup a stderr logger because ProgressLogger uses the `log` crate
    // to printout
    stderrlog::new()
        .verbosity(3)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let graph = load_bidirectional(PathBuf::from(args.graph))
        .context("Could not load graph")?
        .init_properties()
        .load_properties(|properties| properties.load_maps::<GOVMPH>())
        .context("Could not load graph properties")?
        .load_properties(|properties| properties.load_label_names())
        .context("Could not load label names")?
        .load_properties(|properties| properties.load_timestamps())
        .context("Could not load timestamps")?
        .load_labels()
        .context("Could not load labels")?;

    // Lookup SWHID
    info!("looking up SWHID {} ...", args.swhid);
    let node_id = graph
        .properties()
        .node_id(&*args.swhid)
        .context("Unknown SWHID")?;
    info!("obtained node ID {node_id} ...");

    // Setup a queue and a visited bitmap for the visit
    let num_nodes = graph.num_nodes();
    let mut visited = bitvec![u64, Lsb0; 0; num_nodes];
    let mut queue: VecDeque<usize> = VecDeque::new();
    assert!(node_id < num_nodes);
    queue.push_back(node_id);

    // Setup the progress logger for
    let mut pl = ProgressLogger::default().display_memory();
    let mut visited_nodes = 0;
    pl.item_name = "node";
    pl.local_speed = true;
    pl.expected_updates = Some(num_nodes);
    pl.start("visiting graph ...");

    let mut min_timestamp = i64::MAX;
    let mut earliest_revision = None;

    // this code only looks for the predeccessors of the node regardless of finding the match
    

    while let Some(current_node) = queue.pop_front() {
        let visited_swhid = graph.properties().swhid(current_node);
        debug!("{visited_swhid}");
        visited_nodes += 1;
        let mut predecessors = graph.predecessors(current_node);
        while let Some(pred) = predecessors.next() {
            debug!("  Predecessor: {}", graph.properties().swhid(pred));
            let node_type = graph.properties().node_type(pred);
            if !visited[pred] && node_type != NodeType::Revision {
                queue.push_back(pred);
                visited.set(pred as _, true);
                pl.light_update();
            }
            if node_type == NodeType::Revision {
                let timestamp = graph.properties().author_timestamp(pred).expect("missing ts");
                if timestamp < min_timestamp {
                    min_timestamp = timestamp;
                    earliest_revision = Some(pred);
                }
                debug!("  revision timestamp: {:?}", timestamp);
            }
        }
    }

    if let Some(node_id) = earliest_revision {
        info!("Earliest revision: {}", graph.properties().swhid(node_id));
        info!("Timestamp: {}", min_timestamp);
    }

    pl.done();
    info!("visit completed after visiting {visited_nodes} nodes.");

    Ok(())
}
