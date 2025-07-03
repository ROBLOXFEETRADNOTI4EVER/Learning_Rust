use bevy::{color::{self, color_difference}, ecs::event, input::{keyboard::Key, mouse::AccumulatedMouseMotion}, math::primitives::Sphere, prelude::*, transform, window::{CursorGrabMode, PrimaryWindow}};
use bevy::window::WindowFocused;
use bevy::input::common_conditions::input_just_released;
use bevy::input::mouse;
use bevy::math::primitives::Cuboid;
use rand::Rng;
fn main() {
 let mut app =    App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_map, spawn_camera, ));
    app.add_systems(Update,(player_look, player_move.after(player_look), focus_events,
         toogle_grab.run_if(input_just_released(KeyCode::Escape)),
         clickback.run_if(input_just_released(MouseButton::Left)),
         clickback.run_if(input_just_released(MouseButton::Right)),
         spawn_ball

        ));
    app.add_observer(apply_grab);
    app.insert_resource(IsGrabbed(true));

        app.run();
}

const SPEED: f32 = 20.;

#[derive(Component)]
struct  Player;

#[derive(Resource)]
struct IsGrabbed(bool);


#[derive(Component)]
struct  PlayerPosition{
    x_axis: f32 ,
    y_axis: f32,
    z_axis: f32,
}

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
    commands.spawn((
        Camera3d::default(),
        Player,
        PlayerPosition {
            x_axis: 0.0,
            y_axis: 0.0,
            z_axis: 0.0,
        }
    ));
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


fn spawn_ball(
    player_query: Query<(&Transform, &PlayerPosition), With<Player>>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window,With<PrimaryWindow>>,

){
    if input.just_pressed(MouseButton::Middle){
        info!("Middle key pressed ");
        if let Ok((transform,position)) = player_query.single(){
            info!("Player positon x {} y {} z {}",position.x_axis,position.y_axis,position.z_axis);
            let ball_mesh = mesh_assets.add(Sphere::new(1.));
            let square_mesh = mesh_assets.add(Cuboid::new( 1.,1.,1.));

            let i: u32 = rand::thread_rng().gen_range(0..=255);

                let color = Color::hsl((i as f32 / 16.) * 360., 1. + i as f32 / 60., 0.5);
            
            let ball_meterial: Handle<StandardMaterial> = material_assets.add(StandardMaterial{
                base_color: color,
                ..Default::default()
            });
            use EulerRot::YXZ;
            let (mut yaw,mut pitch,_) = transform.rotation.to_euler(YXZ);
            let dt = time.delta_secs();
            let sensitivity = 100. / 2. / ( window.width().min(window.height()));
            let player_pos = transform.translation;
            let forward_direction = transform.forward().as_vec3();
            let spawn_distance = 5.0;
            let spawn_position = player_pos + (forward_direction * spawn_distance);
            
            commands.spawn((
                Transform::from_translation(spawn_position),
                Mesh3d(ball_mesh.clone()),
                MeshMaterial3d(ball_meterial),
            ));
          
        
        }}
       
      
    }
  


// #[derive(Resource)]
// struct  PlayerSpeed(f32);


fn player_move(
    mut player: Single<(&mut Transform, &mut PlayerPosition), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,){  
        let (mut transform, mut position) = player.into_inner();
    
        let mut delta = Vec3::ZERO;
        if input.pressed(KeyCode::KeyA){
            delta.x -= 1.;
            info!("KEy pressed A");
        }
        if input.pressed(KeyCode::KeyD){
            delta.x += 1.;
            info!("Key pressed D ")
        }
        if input.pressed(KeyCode::KeyW){
            delta.z += 1.;
            info!("KEy pressed w");

        }
        if input.pressed(KeyCode::KeyS){
            delta.z -= 1.;
            info!("KEy pressed S");

            
        }

        let forward = transform.forward().as_vec3() * delta.z;
        let right = transform.right().as_vec3() * delta.x;
        let mut to_move = forward + right;
        to_move.y = 0.;
        to_move = to_move.normalize_or_zero();
        // info!("{forward}, {right}");
        if to_move.length_squared() > 0.0 {
            info!("Player moved");
        }
        transform.translation  += to_move * time.delta_secs() * SPEED;
        position.x_axis = transform.translation.x;
        position.y_axis = transform.translation.y;
        position.z_axis = transform.translation.z;

}
