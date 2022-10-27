use bevy::prelude::*;

pub(crate) fn mouse_pos_to_global(window: &Window, (camera, camera_transform): (&Camera, &GlobalTransform)) -> Vec2 {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (window.cursor_position().unwrap() / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
    return world_pos;
}