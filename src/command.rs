use crate::message::{Message, Messages};
use std::collections::HashMap;
use std::any::{Any, TypeId};

/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
/// Command is a simple marker trait for command structs.  These are commands that are issued and handled
/// by a command handler.  They are things we can say "no" to.
pub trait Command: Message {
    fn kind(&self) -> &'static str;
}

/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
/// Commands is a simple marker trait that should be used on an enum of Command types.
pub trait Commands: Messages {}

/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
/// Command handler will handle a single command only.
pub trait Handles<T: Message> {
    type Error;

    fn handle(&mut self, msg: &T) -> Result<(), Self::Error>;

    fn handles(&self) -> &'static str;
}

/// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
/// Command handler will handle a single command only.
pub trait HandlesAll<T: Messages> {
    type Error;

    fn handle(&mut self, msg: &T) -> Result<(), Self::Error>;
}

//// OLD DESIGN - SAVING FOR POSTERITY.
///// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
///// Command is a simple marker trait for command structs.  These are commands that are issued and handled
///// by a command handler.  They are things we can say "no" to.
//pub trait Command {
//    fn kind(&self) -> &'static str;
//}
//
///// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
///// Command handler will handle a single command only.
//pub trait CommandHandler<T: Command> {
//    type Error;
//
//    fn handle(&mut self, command: T) -> Result<(), Self::Error>;
//
//    fn handles(&self) -> String;
//}

// Note: These constructs are likely unnecessary.  They don't provide much more than what you
// could just design yourself in a project.
///// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
///// For use in downcasting in incoming command into a specific type for CommandHandler.
//pub trait GenericHandler {
//    type Error;
//
//    fn handle(&mut self, command: Box<dyn Any>) -> Result<(), Self::Error>;
//
//    fn handles(&self) -> String;
//}
//
//
///// NOTE: THIS IS A WORK IN PROGRESS AND NOT READY FOR USE.
///// Handles all commands for a given system, including command registration.
//pub trait MassHandler {
//    type Error;
//
//    fn handle<T: Command + 'static>(&mut self, command: T) -> Result<(), Self::Error>;
//
//    fn register<T: Command + 'static, U: CommandHandler<T> + 'static>(&mut self, handler: U) -> Result<(), Self::Error>;
//}


