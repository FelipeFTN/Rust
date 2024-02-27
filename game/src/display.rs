pub fn clear() {
    println!(
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    );
    // println!("");
}

pub fn hide_cursor() {
    println!("{}", termion::cursor::Hide);
}

pub fn show_cursor() {
    println!("{}", termion::cursor::Show);
}
