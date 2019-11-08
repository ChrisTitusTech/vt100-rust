#![allow(clippy::cognitive_complexity)]

#[test]
fn colors() {
    let mut parser = vt100::Parser::new(24, 80);

    parser.process(b"foo\x1b[31mbar");

    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), "f");
    assert_eq!(
        parser.screen().cell(0, 0).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(0, 0).unwrap().bgcolor(),
        vt100::Color::Default
    );

    assert_eq!(parser.screen().cell(0, 3).unwrap().contents(), "b");
    assert_eq!(
        parser.screen().cell(0, 3).unwrap().fgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(0, 3).unwrap().bgcolor(),
        vt100::Color::Default
    );

    parser.process(b"\x1b[2D\x1b[45mab");

    assert_eq!(parser.screen().cell(0, 4).unwrap().contents(), "a");
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().fgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().bgcolor(),
        vt100::Color::Idx(5)
    );

    parser.process(b"\x1b[m");

    parser.process(b"\x1b[15;15Hfoo\x1b[31mbar\x1b[m");

    assert_eq!(parser.screen().cell(14, 14).unwrap().contents(), "f");
    assert_eq!(
        parser.screen().cell(14, 14).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(14, 14).unwrap().bgcolor(),
        vt100::Color::Default
    );

    assert_eq!(parser.screen().cell(14, 17).unwrap().contents(), "b");
    assert_eq!(
        parser.screen().cell(14, 17).unwrap().fgcolor(),
        vt100::Color::Idx(1)
    );
    assert_eq!(
        parser.screen().cell(14, 17).unwrap().bgcolor(),
        vt100::Color::Default
    );

    parser.process(b"\x1b[2D\x1b[45mab");

    assert_eq!(parser.screen().cell(14, 18).unwrap().contents(), "a");
    assert_eq!(
        parser.screen().cell(14, 18).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(14, 18).unwrap().bgcolor(),
        vt100::Color::Idx(5)
    );

    parser.process(b"\x1b[m\x1b[2J\x1b[H");
    parser.process(b"a\x1b[38;5;123mb\x1b[48;5;158mc");

    assert_eq!(
        parser.screen().cell(0, 0).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(0, 0).unwrap().bgcolor(),
        vt100::Color::Default
    );

    assert_eq!(
        parser.screen().cell(0, 1).unwrap().fgcolor(),
        vt100::Color::Idx(123)
    );
    assert_eq!(
        parser.screen().cell(0, 1).unwrap().bgcolor(),
        vt100::Color::Default
    );

    assert_eq!(
        parser.screen().cell(0, 2).unwrap().fgcolor(),
        vt100::Color::Idx(123)
    );
    assert_eq!(
        parser.screen().cell(0, 2).unwrap().bgcolor(),
        vt100::Color::Idx(158)
    );

    parser.process(b"\x1b[38;2;50;75;100md\x1b[48;2;125;150;175me");

    assert_eq!(
        parser.screen().cell(0, 3).unwrap().fgcolor(),
        vt100::Color::Rgb(50, 75, 100)
    );
    assert_eq!(
        parser.screen().cell(0, 3).unwrap().bgcolor(),
        vt100::Color::Idx(158)
    );

    assert_eq!(
        parser.screen().cell(0, 4).unwrap().fgcolor(),
        vt100::Color::Rgb(50, 75, 100)
    );
    assert_eq!(
        parser.screen().cell(0, 4).unwrap().bgcolor(),
        vt100::Color::Rgb(125, 150, 175)
    );

    parser.process(b"\x1b[m\x1b[2J\x1b[H");
    parser.process(b"\x1b[32;47mfoo");

    assert_eq!(
        parser.screen().cell(0, 1).unwrap().fgcolor(),
        vt100::Color::Idx(2)
    );
    assert_eq!(
        parser.screen().cell(0, 1).unwrap().bgcolor(),
        vt100::Color::Idx(7)
    );

    parser.process(b"\x1b[2J\x1b[H");
    parser.process(b"\x1b[39mfoo");

    assert_eq!(
        parser.screen().cell(0, 1).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(0, 1).unwrap().bgcolor(),
        vt100::Color::Idx(7)
    );

    parser.process(b"\x1b[2J\x1b[H");
    parser.process(b"\x1b[49mfoo");

    assert_eq!(
        parser.screen().cell(0, 1).unwrap().fgcolor(),
        vt100::Color::Default
    );
    assert_eq!(
        parser.screen().cell(0, 1).unwrap().bgcolor(),
        vt100::Color::Default
    );

    parser.process(b"\x1b[m\x1b[2J\x1b[H");
    parser.process(b"\x1b[92;107mfoo");

    assert_eq!(
        parser.screen().cell(0, 1).unwrap().fgcolor(),
        vt100::Color::Idx(10)
    );
    assert_eq!(
        parser.screen().cell(0, 1).unwrap().bgcolor(),
        vt100::Color::Idx(15)
    );
}

#[test]
fn attrs() {
    let mut parser = vt100::Parser::new(24, 80);

    parser.process(b"f\x1b[1mo\x1b[3mo\x1b[4mo\x1b[7mo");
    assert!(!parser.screen().cell(0, 0).unwrap().bold());
    assert!(!parser.screen().cell(0, 0).unwrap().italic());
    assert!(!parser.screen().cell(0, 0).unwrap().underline());
    assert!(!parser.screen().cell(0, 0).unwrap().inverse());
    assert!(parser.screen().cell(0, 1).unwrap().bold());
    assert!(!parser.screen().cell(0, 1).unwrap().italic());
    assert!(!parser.screen().cell(0, 1).unwrap().underline());
    assert!(!parser.screen().cell(0, 1).unwrap().inverse());
    assert!(parser.screen().cell(0, 2).unwrap().bold());
    assert!(parser.screen().cell(0, 2).unwrap().italic());
    assert!(!parser.screen().cell(0, 2).unwrap().underline());
    assert!(!parser.screen().cell(0, 2).unwrap().inverse());
    assert!(parser.screen().cell(0, 3).unwrap().bold());
    assert!(parser.screen().cell(0, 3).unwrap().italic());
    assert!(parser.screen().cell(0, 3).unwrap().underline());
    assert!(!parser.screen().cell(0, 3).unwrap().inverse());
    assert!(parser.screen().cell(0, 4).unwrap().bold());
    assert!(parser.screen().cell(0, 4).unwrap().italic());
    assert!(parser.screen().cell(0, 4).unwrap().underline());
    assert!(parser.screen().cell(0, 4).unwrap().inverse());

    parser.process(b"\x1b[m");
    parser.process(b"\x1b[2J\x1b[H");
    parser.process(b"\x1b[1;4mf");
    assert!(parser.screen().cell(0, 0).unwrap().bold());
    assert!(!parser.screen().cell(0, 0).unwrap().italic());
    assert!(parser.screen().cell(0, 0).unwrap().underline());
    assert!(!parser.screen().cell(0, 0).unwrap().inverse());

    parser.process(b"\x1b[22mo\x1b[24mo");
    assert!(!parser.screen().cell(0, 1).unwrap().bold());
    assert!(!parser.screen().cell(0, 1).unwrap().italic());
    assert!(parser.screen().cell(0, 1).unwrap().underline());
    assert!(!parser.screen().cell(0, 1).unwrap().inverse());
    assert!(!parser.screen().cell(0, 2).unwrap().bold());
    assert!(!parser.screen().cell(0, 2).unwrap().italic());
    assert!(!parser.screen().cell(0, 2).unwrap().underline());
    assert!(!parser.screen().cell(0, 2).unwrap().inverse());

    parser.process(b"\x1b[1;3;4;7mo");
    assert!(parser.screen().cell(0, 3).unwrap().bold());
    assert!(parser.screen().cell(0, 3).unwrap().italic());
    assert!(parser.screen().cell(0, 3).unwrap().underline());
    assert!(parser.screen().cell(0, 3).unwrap().inverse());
}
