

use bevy::prelude::*;



#[derive(Component)]
pub struct Person;



#[derive(Component)]
pub struct Name(String);



pub fn add_people(mut commands: Commands) {
  commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
  commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
  commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}


pub struct GreetTimer(Timer);

pub fn greet_people(
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>, 
    query: Query<&Name, With<Person>>
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
  fn build(&self, app: &mut App) {
      
    // create timer - repeats ech 2 seconds
    // the reason we call from_seconds with the true flag is to make the timer repeat itself
    let my_timer = GreetTimer(Timer::from_seconds(2.0, true));
    
    // initialize app
    app.insert_resource(my_timer)
        .add_startup_system(add_people)
        .add_system(greet_people);
  }
}


