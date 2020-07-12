use ezmsg::Msg;

#[test]
fn simple() {
  let mut msg = Msg::new();

  msg.set_topic("SomeTopic").unwrap();
  assert_eq!(msg.get_topic().unwrap(), "SomeTopic");

  msg.add_str("Foo", "bar");
  assert_eq!(msg.get_param("Foo").unwrap(), "bar");

  assert_eq!(msg.get_param("Moo"), None);
}

#[test]
fn integer() {
  let mut msg = Msg::new();

  msg.set_topic("SomeTopic").unwrap();
  assert_eq!(msg.get_topic().unwrap(), "SomeTopic");

  msg.add_str("Num", "64");
  assert_eq!(msg.get_int::<u16>("Num").unwrap(), 64);
}


#[test]
fn size() {
  let mut msg = Msg::new();

  msg.add_param("Num", 7 as usize);
  assert_eq!(msg.get_int::<usize>("Num").unwrap(), 7);
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
