use i3ipc::{I3Connection, I3EventListener, Subscription, event::Event, event::inner::WindowChange};
use time::OffsetDateTime;
use log::{error};

mod database;
use rusqlite::{params, Connection, Result};


fn main() {
	let db_connection = database::open_database("time_tracking_data.db").expect("Unable to open database");

	loop {	
		let err = connect_i3(&db_connection);
		error!("Connection to i3 lost: {}", err);
	}
}



fn connect_i3(db_connection: &Connection) -> i3ipc::MessageError {
	let mut listener = I3EventListener::connect().unwrap();
	listener.subscribe(&[Subscription::Window]).unwrap();
	
	for event in listener.listen() {
		match event {
			Ok(event) => {

				let entry = create_time_entry(&event);
				
				let time_string = entry.event_time.format("%F %T");
				db_connection.execute("
					INSERT INTO window_events (event_source, event_time, window_area, window_class, window_name)
					    VALUES (?1, ?2, ?3, ?4, ?5)",
					params![entry.event_source.to_string(), time_string, entry.window_area, entry.window_class, entry.window_name],
				).unwrap();
			},
			Err(err) => {
				return err
			}
		}
	}
	unreachable!();
}

fn create_time_entry(event: &i3ipc::event::Event) -> database::TimeEntry {
	match event {
		Event::WindowEvent(event) => {
			let empty = "".to_string();

			let window_properties = std::collections::HashMap::default();
			let window_properties = event.container.window_properties.as_ref().unwrap_or(&window_properties);
			
			
			let window_name = event.container.name.clone().unwrap_or(empty.clone());
			let window_class = window_properties.get(&i3ipc::reply::WindowProperty::Class).unwrap_or(&empty).clone();
			
			let rect = event.container.window_rect;
			
			let entry = database::TimeEntry {
				event_time: OffsetDateTime::now(),
				event_source: database::WindowEvent::from_i3_event(&event.change),
				window_area: rect.2 * rect.3, // Width, height
				window_name,
				window_class: window_class.to_string()
			};
			entry
		}
		_ => {
			unimplemented!();
		}
	}
}

impl database::WindowEvent {
	fn from_i3_event(change: &WindowChange) -> Self {

		let out = match change {
			WindowChange::New => database::WindowEvent::Open,
			WindowChange::Close => database::WindowEvent::Close,
			WindowChange::Focus => database::WindowEvent::Focus,
			WindowChange::Title => database::WindowEvent::TitleChange,
			WindowChange::FullscreenMode => database::WindowEvent::Move,
			WindowChange::Move => database::WindowEvent::Move,
			WindowChange::Floating => database::WindowEvent::Move,
			WindowChange::Urgent => database::WindowEvent::Other,
			WindowChange::Unknown => database::WindowEvent::Other,
			
		};
		println!("{:?} {:?}", change, out);
		out
	}
}
