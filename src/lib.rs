#![allow(non_snake_case)]

//use std::sync::Arc;
#[macro_use]
extern crate lazy_static;

extern crate colored;
use colored::Colorize;

use std::sync::Mutex;
use std::sync::MutexGuard;

use std::io::Write;

use std::fs::File;


//pub static LOG_FILE: Arc< Mutex< File > > = Arc::new( Mutex::new( File::create("log.txt").expect( "Impossible de créer le fichier de log") ) );

lazy_static! {
	static ref LOG_FILE: Mutex< File > = Mutex::new( File::create("log.txt").expect( "Impossible de créer le fichier de log") );
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
fn LogWrite( _str: &str ) {
	let lock : MutexGuard< File > = LOG_FILE.lock().expect( "Impossible de verrouiller le fichier de log" );
	let mut file : &File = &*lock;
	file.write_all( _str.as_bytes() ).expect( "Erreur d'écriture dans le fichier de log" );
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
pub fn Log( _str: &str ) {
	print!( "{}", _str );
	LogWrite( _str );
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
pub fn LogWarning( _str: &str ) {
	print!( "{}", _str.yellow().italic() );
	LogWrite( _str );
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
pub fn LogError( _str: &str ) {
	print!( "{}", _str.red().bold() );
	LogWrite( _str );
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! Printf {
	($($arg:tt)*) => {
		Log( format!($($arg)*).as_str() );
	};
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! Warning {
	($($arg:tt)*) => {
		let mut outputStr : String = format!($($arg)*);
		outputStr = "[Warning] ".to_owned() + outputStr.as_str();
		outputStr = outputStr + "\n";
		LogWarning( &outputStr );
	};
}

// -----------------------------------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------------
#[macro_export]
macro_rules! Error {
	($($arg:tt)*) => {
		let mut outputStr : String = format!($($arg)*);
		outputStr = "[Erreur] ".to_owned() + outputStr.as_str();
		outputStr = outputStr + "\n";
		LogError( &outputStr );
	};
}