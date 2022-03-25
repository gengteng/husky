#[macro_export]
macro_rules! test_print {
    ($($v:expr),*) => {
        #[cfg(test)]
        eprintln!(r#"
-------------------------------------------------------------------
{}{}:{}{}:{}
{}"#,
        print_utils::GREEN,
        file!(),
        print_utils::YELLOW,
        line!(),
        print_utils::RESET,
        show!($($v),*),
    )};
}
#[macro_export]
macro_rules! p {
    ($($v:expr),*) => {
        eprintln!(r#"
-------------------------------------------------------------------
{}{}:{}{}:{}
{}"#,
        print_utils::GREEN,
        file!(),
        print_utils::YELLOW,
        line!(),
        print_utils::RESET,
        print_utils::show!($($v),*),
    )};
}

#[macro_export]
macro_rules! ep {
  ($($v:expr),*) => {eprintln!("src: {}:{}\n  {}\n",
  file!(),
  line!(),
  print_utils::eshow!($($v),*))};
}

#[macro_export]
macro_rules! esp {
  ($($v:expr),*) => {eprintln!("src: {}:{}\n  {}\n",
  file!(),
  line!(),
  print_utils::esimple_show!($($v),*))};
}

#[macro_export]
macro_rules! ep_once {
  ($($v:expr),*) => {common::do_once(||eprintln!("{}\n\t\tsrc: {}:{}",
  print_utils::eshow!($($v),*),
  file!(),
  line!()))};
}

#[macro_export]
macro_rules! msg_once {
    ($msg:expr) => {
        common::do_once(|| eprintln!("[message] {}\n\t\tsrc: {}:{}", $msg, file!(), line!()))
    };
}

#[macro_export]
macro_rules! epin {
    () => {
        eprintln!("[pin] src: {}:{}.", file!(), line!());
    };
}