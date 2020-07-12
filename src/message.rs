use std::fmt;
use std::str::FromStr;
use std::collections::HashMap;

#[cfg(feature = "bytes")]
use bytes::{BytesMut, BufMut};

use crate::err::Error;

use crate::validators::validate_topic;

#[derive(Debug,Clone)]
pub struct Msg {
  topic: Option<String>,
  params: HashMap<String, String>
}


impl Msg {
  /// Create a new message object, with an unset topic.
  ///
  /// Note that a message object without a topic is invalid.  `set_topic` must
  /// be called to set a topic to make the message object valid.
  pub fn new() -> Self {
    Msg { topic: None, params: HashMap::new() }
  }

  /// Create a new mesage object with a topic.
  pub fn new_topic(topic: &str) -> Result<Self, Error> {
    validate_topic(topic)?;
    Ok(Msg { topic: Some(topic.to_string()), params: HashMap::new() })
  }

  /// Get a reference to the internal HashMap.
  pub fn get_params(&self) -> &HashMap<String, String> {
    &self.params
  }

  /// Set topic for message.
  ///
  /// Overwrites current topic is one has already been set.
  pub fn set_topic(&mut self, topic: &str) -> Result<(), Error> {
    validate_topic(topic)?;
    self.topic = Some(topic.to_string());
    Ok(())
  }

  /// Get a reference to the topic string, or None if topic is not been set.
  pub fn get_topic(&self) -> Option<&str> {
    if let Some(t) = &self.topic {
      Some(t)
    } else {
      None
    }
  }

  /// Add a parameter to the message.
  pub fn add_param<T: ToString, U: ToString>(
      &mut self,
      key: T,
      value: U
  ) {
    self.params.insert(key.to_string(), value.to_string());
  }

  /// Add a string parameter to the message.
  pub fn add_str<T: ToString, U: ToString>(
      &mut self,
      key: T,
      value: U
  ) {
    self.params.insert(key.to_string(), value.to_string());
  }

  pub fn get_param(&self, key: &str) -> Option<&str> {
    let kv = self.params.get_key_value(key);
    if let Some((_k, v)) = kv {
      return Some(v);
    }
    None
  }

  pub fn get_str(&self, key: &str) -> Option<&str> {
    let kv = self.params.get_key_value(key);
    if let Some((_k, v)) = kv {
      return Some(v);
    }
    None
  }

  pub fn get_int<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!("Unable to parse numeric value from \
parameter '{}'", key)));
    }
    Err(Error::KeyNotFound(key.to_string()))
  }


  pub fn serialize(&self) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    if let Some(ref h) = self.topic {
      // Copy topic
      let b = h.as_bytes();
      for a in b {
        buf.push(*a);
      }
      buf.push(b'\n');
    } else {
      return Err(Error::BadFormat("Missing heading".to_string()));
    }

    for (key, value) in &self.params {
      let k = key.as_bytes();
      let v = value.as_bytes();
      for a in k {
        buf.push(*a);
      }
      buf.push(b' ');
      for a in v {
        buf.push(*a);
      }
      buf.push(b'\n');
    }

    buf.push(b'\n');

    Ok(buf)
  }

  pub fn clear(&mut self) {
    self.topic = None;
    self.params.clear();
  }

  /// Write the Msg to a buffer.
  #[cfg(feature = "bytes")]
  pub fn encoder_write(
      &self,
      buf: &mut BytesMut
  ) -> Result<(), Error> {
    if self.topic.is_none() {
      return Err(Error::SerializeError("Missing Msg topic".to_string()));
    }

    // Calculate the required buffer size
    let mut size = 0;
    if let Some(ref h) = self.topic {
      size += h.len() + 1;      // including '\n'
    }
    for (key, value) in &self.params {
      size += key.len() + 1;    // including ' '
      size += value.len() + 1;  // including '\n'
    }
    size += 1;    // terminating '\n'

    // Reserve space
    buf.reserve(size);

    // Write data to output buffer
    if let Some(ref b) = self.topic {
      buf.put(b.as_bytes());
    }
    buf.put_u8(b'\n');

    for (key, value) in &self.params {
      buf.put(key.as_bytes());
      buf.put_u8(b' ');
      buf.put(value.as_bytes());
      buf.put_u8(b'\n');
    }
    buf.put_u8(b'\n');

    Ok(())
  }
}


impl fmt::Display for Msg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut plist = String::new();

    for (key, value) in &self.params {
      plist.push_str(key);
      plist.push('=');
      plist.push_str(value);
      plist.push(',')
    }

    if plist.len() > 0 {
      plist.truncate(plist.len()-1);
    }

    let topic: &str = match &self.topic {
      Some(s) => s.as_ref(),
      None => &"<None>"
    };

    write!(f, "{}:{{{}}}", topic, plist)
  }
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
