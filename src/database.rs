/// Large interactions with the database (Eg opening/closing) take
/// place in this file, as well as the structs that are stored inside
/// it.

use rusqlite::{params, Connection};
use time::OffsetDateTime;

/// Something that a user can do as part of an event
#[derive(Debug)]
pub enum WindowEvent {
	Open,
	Close,
	Move, // Anything to do with window management: fullscreening, layout switching etc.
	Focus,
	TitleChange,
	Other,
}

impl WindowEvent {
	pub fn to_string(&self) -> String {
		match self {
			Open => "open",
			Close => "close",
			Move => "move",
			Focus => "focus",
			TitleChange => "title_change",
			Other => "other",
		}.to_string()
	}
}


/// Record what a user is doing at an instant in time.
#[derive(Debug)]
pub struct TimeEntry {
	/// The time that this event occurred
	pub event_time: OffsetDateTime,
	/// What this event is (eg is the user moving windows, opening a new
	/// program etc.
	pub event_source: WindowEvent,
	
	/// The area on the screen that the window occupies. This can
	/// be used to compute efficiency
	pub window_area: i32,
	
	/// The name of the window the user is interacting with. Often this
	/// contains information such as the website currently visited, the
	/// path open in the terminal etc.
	pub window_name: String,
	
	/// What type of window the user is interacting with. Most programs
	/// set this to be the name of the program, and it can be used to
	/// group multiple actions.
	pub window_class: String,
}


#[derive(Debug)]
pub enum DatabaseError {
	OpenFailed(rusqlite::Error),
	EnsureSchemaFailed
}


/// Opens the database
pub fn open_database(path: &str) -> Result<Connection, DatabaseError> {
	let db_connection = Connection::open(path).map_err(|e| DatabaseError::OpenFailed(e))?;
	db_connection.execute(
		"CREATE TABLE IF NOT EXISTS window_events (
		    id                  INTEGER PRIMARY KEY,
		    event_source        TEXT NOT NULL,
		    event_time          TEXT NOT NULL,
		    window_area   INT,
		    window_class        TEXT,
		    window_name         TEXT
		)",
		params![],
	).expect("Failed to create table");
	
	Ok(db_connection)
}
