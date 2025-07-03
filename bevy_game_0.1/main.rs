use bevy::{color::{self, color_difference}, ecs::event, input::{keyboard::Key, mouse::AccumulatedMouseMotion}, math::primitives::Sphere, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy::window::WindowFocused;
use bevy::input::common_conditions::input_just_released;
use bevy::input::mouse;
use bevy::math::primitives::Cuboid;
fn main() {
 let mut app =    App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_map, spawn_camera, ));
    app.add_systems(Update,(player_look, focus_events,
         toogle_grab.run_if(input_just_released(KeyCode::Escape)),
         clickback.run_if(input_just_released(MouseButton::Left)),
         clickback.run_if(input_just_released(MouseButton::Right)),

        ));
    app.add_observer(apply_grab);
    app.insert_resource(IsGrabbed(true));

        app.run();
}


#[derive(Component)]
struct  Player;

#[derive(Event,Deref)]

struct  GrabEvent(bool);
// To do make it so when i click it flips it to false so it grabs it

fn toogle_grab(
    mut is_grabbed: ResMut<IsGrabbed>,
    mut commands: Commands,
) {
    is_grabbed.0 = !is_grabbed.0; // flip the state
    commands.trigger(GrabEvent(is_grabbed.0));
}

fn clickback(    mut is_grabbed: ResMut<IsGrabbed>,
    mut commands: Commands,){
        is_grabbed.0 = true;
        commands.trigger(GrabEvent(is_grabbed.0));

    }

fn apply_grab(
    grab: Trigger<GrabEvent>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
){
    use bevy::window::CursorGrabMode;
    if **grab{
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else{
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }



}
#[derive(Resource)]
struct IsGrabbed(bool);


fn focus_events(
    mut events: EventReader<WindowFocused>,
    window: Single<&Window,With<PrimaryWindow>>,
    mut commands: Commands,
){
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}


fn spawn_camera(mut commands: Commands){
    commands.spawn((Camera3d::default(),Player));
}

fn player_look(  mut player: Single<&mut Transform, With<Player>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window,With<PrimaryWindow>>,
    mut is_grabbed: ResMut<IsGrabbed>,

    ) {
    if is_grabbed.0 == false {
        return
    }    
    let dt = time.delta_secs();
    let sensitivity = 100. / 2. / ( window.width().min(window.height()));
    if !window.focused{
        return;
    }
    use EulerRot::YXZ;
    let (mut yaw,mut pitch,_) = player.rotation.to_euler(YXZ);
    pitch -= mouse_motion.delta.y * dt * sensitivity;
    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch = pitch.clamp(-1.57, 1.57);
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);

}

fn spawn_map(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>,mut meterial_assets: ResMut<Assets<StandardMaterial>>){
    commands.spawn((DirectionalLight::default()));
    let ball_mesh = mesh_assets.add(Sphere::new(1.));
    let square_mesh = mesh_assets.add(Cuboid::new( 1.,1.,1.));
    for i in 0..20{
        let color = Color::hsl((i as f32 / 16.) * 360., 1., 0.5);
    
    let ball_meterial: Handle<StandardMaterial> = meterial_assets.add(StandardMaterial{
        base_color: color,
        ..Default::default()
    });
    let sqaure_meterial: Handle<StandardMaterial> = meterial_assets.add(StandardMaterial{
        base_color: color,
        ..Default::default()
    });
    commands.spawn((
        Transform::from_translation(Vec3::new((-8. + i as f32) * 2., 0., -50.0)),
        Mesh3d(ball_mesh.clone()),
        MeshMaterial3d(ball_meterial),
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new((-8. + i as f32) * 2., 4., -20.0)),
        Mesh3d(square_mesh.clone()),
        MeshMaterial3d(sqaure_meterial),
    ));
}
}


