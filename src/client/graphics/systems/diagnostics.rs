use bevy::{
    diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub fn diagnostics(diagnostics: Res<Diagnostics>) {
    let mut fps = None;
    let mut entity_count = None;

    for diagnostic in diagnostics.iter() {
        if diagnostic.id == FrameTimeDiagnosticsPlugin::FPS {
            fps = diagnostic.average();
        } else if diagnostic.id == EntityCountDiagnosticsPlugin::ENTITY_COUNT {
            entity_count = diagnostic.value();
        }
    }

    info!(
        target: "stats",
        "fps: {fps_value:>.2}, entity_count: {entity_count:>.2}",
        fps_value = fps.unwrap_or(-1.0),
        entity_count = entity_count.unwrap_or(-1.0),
    );
}
