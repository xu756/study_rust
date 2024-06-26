use scrap::Display;
fn screen_record() {
    let displays = Display::all().unwrap();

    for (i, display) in displays.iter().enumerate() {
        println!(
            "Display {} [{}x{}]",
            i + 1,
            display.width(),
            display.height()
        );
    }
}

#[test]
fn test_screen_record() {
    screen_record();
}
