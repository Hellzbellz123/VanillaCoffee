use bevy::prelude::*;
use std::{fs, path::{Path, PathBuf}};
use bevy_mod_debugdump::{render_graph, render_graph_dot, schedule_graph, schedule_graph_dot};

/// dumps scheduling graphs for given App
pub fn debug_dump_graphs(app: &mut App) {

    let target = Path::new(".schedule");
    match target.try_exists() {
        Err(error) => {
            warn!("problem with {:?} directory: {}", target, error);
        }
        Ok(exists) => {
            if !exists {
                warn!(
                    "Not dumping schedules because {:?} directory does not exist",
                    target
                );
                warn!(
                    "Create {:?} directory in cwd too dump schedule graphs",
                    target
                );
                return;
            }
            warn!("Dumping graphs");

            let schedule_theme = schedule_graph::settings::Style::dark_github();
            let render_theme = render_graph::settings::Style::dark_github();

            let settings = schedule_graph::Settings {
                ambiguity_enable: false,
                ambiguity_enable_on_world: false,
                style: schedule_theme,
                collapse_single_system_sets: true,
                ..Default::default()
            };

            let render_graph_settings = render_graph::Settings {
                style: render_theme,
            };
            let pre_startup_graph = schedule_graph_dot(app, PreStartup, &settings);
            let main_startup_graph = schedule_graph_dot(app, Startup, &settings);
            let post_startup_graph = schedule_graph_dot(app, PostStartup, &settings);
            let first_schedule = schedule_graph_dot(app, First, &settings);
            let pre_update_schedule = schedule_graph_dot(app, PreUpdate, &settings);
            let main_update_schedule = schedule_graph_dot(app, Update, &settings);
            let post_update_schedule = schedule_graph_dot(app, PostUpdate, &settings);
            let last_schedule = schedule_graph_dot(app, Last, &settings);
            let render_graph = render_graph_dot(app, &render_graph_settings);


            write_graphs(
                target.to_path_buf(),
                (
                    pre_startup_graph,
                    main_startup_graph,
                    post_startup_graph,
                    first_schedule,
                    pre_update_schedule,
                    main_update_schedule,
                    post_update_schedule,
                    last_schedule,
                    render_graph,
                ),
            );
        }
    }
}

/// dumps schedule as a graph
fn write_graphs(
    folder: PathBuf,
    dotfiles: (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ),
) {
    let (
        pre_startup_graph,
        main_startup_graph,
        post_startup_graph,
        first_schedule,
        pre_update_schedule,
        main_update_schedule,
        post_update_schedule,
        last_schedule,
        render_graph,
    ) = dotfiles;

    match fs::write(folder.join("0-pre_startup_schedule.dot"), pre_startup_graph) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("1-main_startup_schedule.dot"),
        main_startup_graph,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(folder.join("2-post_startup_graph.dot"), post_startup_graph) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(folder.join("3-first_schedule.dot"), first_schedule) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("4-pre_update_schedule.dot"),
        pre_update_schedule,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("5-main_update_schedule.dot"),
        main_update_schedule,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(
        folder.join("6-post_update_schedule.dot"),
        post_update_schedule,
    ) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
    match fs::write(folder.join("7-last_schedule.dot"), last_schedule) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }

    match fs::write(folder.join("Z-render_graph.dot"), render_graph) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
}
