use crate::prelude::*;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};

#[cfg(target_family = "windows")]
const SCALE_FACTOR: f32 = -0.05;

#[cfg(not(target_family = "windows"))]
const SCALE_FACTOR: f32 = 0.001;

#[cfg(target_family = "windows")]
const DRAG_FACTOR: f32 = 0.005;

#[cfg(not(target_family = "windows"))]
const DRAG_FACTOR: f32 = 0.01;

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
    is_dragging: bool,
}

fn setup(mut commands: Commands) {
    // 添加相机
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCamera {
            radius: EARTH_RADIUS * 5.,
            azimuthal_angle: 0.,
            polar_angle: 0.,
            is_dragging: false,
        });
}

fn orbit_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    for (mut orbit_camera, mut transform) in query.iter_mut() {
        // 处理鼠标按下和释放事件
        for event in mouse_button_input_events.read() {
            if event.button == MouseButton::Left {
                orbit_camera.is_dragging = event.state.is_pressed();
            }
        }

        // 如果正在拖动，处理鼠标移动事件
        if orbit_camera.is_dragging {
            for event in mouse_motion_events.read() {
                orbit_camera.azimuthal_angle -= event.delta.x * DRAG_FACTOR;
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
        let x = orbit_camera.radius
            * orbit_camera.polar_angle.cos()
            * orbit_camera.azimuthal_angle.sin();
        let y = orbit_camera.radius * orbit_camera.polar_angle.sin();
        let z = orbit_camera.radius
            * orbit_camera.polar_angle.cos()
            * orbit_camera.azimuthal_angle.cos();

        // 设置相机的位置，并使其始终朝向地球（即原点）
        transform.translation = Vec3::new(x, -z, y);
        transform.look_at(Vec3::ZERO, Vec3::Z);
    }
}
