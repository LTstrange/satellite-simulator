use crate::prelude::*;

use bevy::input::mouse::MouseWheel;

#[cfg(target_os = "windows")]
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
#[cfg(target_os = "windows")]
const SCALE_FACTOR: f32 = -0.05;
#[cfg(target_os = "windows")]
const DRAG_FACTOR: f32 = 0.005;

#[cfg(target_os = "macos")]
use bevy::input::gestures::PinchGesture;
#[cfg(target_os = "macos")]
const SCALE_FACTOR: f32 = 0.8;
#[cfg(target_os = "macos")]
const DRAG_FACTOR: f32 = 0.005;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, orbit_camera_system);
    }
}

#[derive(Component)]
struct OrbitCamera {
    radius: f32,
    azimuthal_angle: f32, // 方位角（绕z轴的旋转）
    polar_angle: f32,     // 极角（与赤道夹角）

    #[cfg(target_os = "windows")]
    is_dragging: bool,
}

fn setup(mut commands: Commands) {
    // 添加相机
    commands.spawn((
        Camera3d::default(),
        OrbitCamera {
            radius: EARTH_RADIUS * 5.,
            azimuthal_angle: 0.,
            polar_angle: 0.,

            #[cfg(target_os = "windows")]
            is_dragging: false,
        },
    ));
}

#[cfg(target_os = "windows")]
fn orbit_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    let (mut orbit_camera, mut transform) = query.single_mut();
    // 处理鼠标按下和释放事件
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left {
            orbit_camera.is_dragging = event.state.is_pressed();
        }
    }

    // 如果正在拖动，处理鼠标移动事件
    if orbit_camera.is_dragging {
        for event in mouse_motion_events.read() {
            orbit_camera.azimuthal_angle += event.delta.x * DRAG_FACTOR;
            orbit_camera.polar_angle += event.delta.y * DRAG_FACTOR;

            // 限制极角在合理范围内，防止翻转
            orbit_camera.polar_angle = orbit_camera.polar_angle.clamp(
                -std::f32::consts::PI / 2.0 + 0.01,
                std::f32::consts::PI / 2.0 - 0.01,
            );
        }
    }

    for event in mouse_wheel_events.read() {
        orbit_camera.radius *= 1. + SCALE_FACTOR * event.y;

        orbit_camera.radius = orbit_camera
            .radius
            .clamp(EARTH_RADIUS * 2.0, EARTH_RADIUS * 20.0); // 限制相机的距离在合理范围内，防止相机超出地球表面
    }

    // 计算相机的新位置
    let x =
        orbit_camera.radius * orbit_camera.polar_angle.cos() * orbit_camera.azimuthal_angle.cos();
    let y =
        -orbit_camera.radius * orbit_camera.polar_angle.cos() * orbit_camera.azimuthal_angle.sin();
    let z = orbit_camera.radius * orbit_camera.polar_angle.sin();

    // 设置相机的位置，并使其始终朝向地球（即原点）
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Z);
}

#[cfg(target_os = "macos")]
fn orbit_camera_system(
    mut pinch_events: EventReader<PinchGesture>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) -> Result {
    let (mut orbit_camera, mut transform) = query.single_mut()?;

    // x-axis rotation
    for event in mouse_wheel_events.read() {
        orbit_camera.azimuthal_angle += event.x * DRAG_FACTOR;
        orbit_camera.polar_angle += event.y * DRAG_FACTOR;

        // 限制极角在合理范围内，防止翻转
        orbit_camera.polar_angle = orbit_camera.polar_angle.clamp(
            -std::f32::consts::PI / 2.0 + 0.01,
            std::f32::consts::PI / 2.0 - 0.01,
        );
    }

    for event in pinch_events.read() {
        orbit_camera.radius *= 1. - SCALE_FACTOR * event.0;

        orbit_camera.radius = orbit_camera
            .radius
            .clamp(EARTH_RADIUS * 2.0, EARTH_RADIUS * 20.0); // 限制相机的距离在合理范围内，防止相机超出地球表面
    }

    // 计算相机的新位置
    let x =
        orbit_camera.radius * orbit_camera.polar_angle.cos() * orbit_camera.azimuthal_angle.cos();
    let y =
        -orbit_camera.radius * orbit_camera.polar_angle.cos() * orbit_camera.azimuthal_angle.sin();
    let z = orbit_camera.radius * orbit_camera.polar_angle.sin();

    // 设置相机的位置，并使其始终朝向地球（即原点）
    transform.translation = Vec3::new(x, y, z);
    transform.look_at(Vec3::ZERO, Vec3::Z);

    Ok(())
}
