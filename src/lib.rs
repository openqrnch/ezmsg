//! A "message", represented by the Msg struct, is an entity that is
//! typically transferred over the network between clients/servers or peers.
//! Each message consist of a mandatory "topic" (a utf-8 string) and zero or
//! more parameters (each of which is a key/value pair (of utf-8 strings).
//!
//! ezmsg does not handle transmission; it only represents the message buffer
//! and provides methods to serialize messages.

mod err;
mod message;
mod validators;

pub use message::Msg;
pub use err::Error;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
