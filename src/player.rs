use bevy::prelude::*;

const PLAYER_SPEED: f32 = 0.25;
const CAMERA_DISTANCE: [f32; 3] = [0.0, 10.0, 10.0];

#[derive(Component)]
pub struct Player;

// PH/ST = Phantom Hourglass / Spirit Tracks
fn phst_movement(
    mut q_player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mb: Res<Input<MouseButton>>,
    wnds: Res<Windows>,
) {
    if mb.pressed(MouseButton::Left) {
        for mut transform in q_player.iter_mut() {
            let wnd = wnds.get_primary().unwrap();

            if let Some(screen_pos) = wnd.cursor_position() {
                let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                info!("Spam spam here's some maybe useful info: NDC = {},{}", ndc.x, ndc.y);

                transform.translation.x += ndc.x * PLAYER_SPEED;
                transform.translation.z -= ndc.y * PLAYER_SPEED;
            }
        }
    }
}

fn camera_tracking(
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut cam_t = q_camera.single_mut();
    let player_t = q_player.single();

    cam_t.translation = player_t.translation + Vec3::from(CAMERA_DISTANCE);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(phst_movement)
            .add_system(camera_tracking);
    }
}
