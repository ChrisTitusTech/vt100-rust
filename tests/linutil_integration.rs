// Integration test to verify vt100-ctt works as expected for linutil usage
// Tests basic parser creation, processing, and screen access

#[test]
fn test_basic_parser_usage() {
    // Create a parser as linutil would
    let mut parser = vt100_ctt::Parser::new(24, 80, 1000);
    
    // Process some basic terminal output
    parser.process(b"Hello, World!\n");
    parser.process(b"\x1b[31mRed text\x1b[0m");
    
    // Verify we can access the screen
    let screen = parser.screen();
    assert!(screen.contents().contains("Hello, World!"));
    assert!(screen.contents().contains("Red text"));
}

#[test]
fn test_parser_with_callbacks() {
    use vt100_ctt::Callbacks;
    
    #[derive(Default)]
    struct TestCallbacks {
        title: String,
        bell_count: usize,
    }
    
    impl Callbacks for TestCallbacks {
        fn set_window_title(&mut self, _screen: &mut vt100_ctt::Screen, title: &[u8]) {
            self.title = String::from_utf8_lossy(title).to_string();
        }
        
        fn audible_bell(&mut self, _screen: &mut vt100_ctt::Screen) {
            self.bell_count += 1;
        }
    }
    
    let callbacks = TestCallbacks::default();
    let mut parser = vt100_ctt::Parser::new_with_callbacks(24, 80, 1000, callbacks);
    
    // Set window title
    parser.process(b"\x1b]0;Test Title\x07");
    assert_eq!(parser.callbacks().title, "Test Title");
    
    // Trigger bell
    parser.process(b"\x07");
    assert_eq!(parser.callbacks().bell_count, 1);
}

#[test]
fn test_screen_cursor_and_colors() {
    let mut parser = vt100_ctt::Parser::new(24, 80, 0);
    
    // Move cursor and write colored text
    parser.process(b"\x1b[10;20H");
    parser.process(b"\x1b[32mGreen\x1b[0m");
    
    let screen = parser.screen();
    let (row, col) = screen.cursor_position();
    assert_eq!(row, 9); // 0-indexed
    assert!(col >= 20);
}

#[test]
fn test_screen_size_changes() {
    let mut parser = vt100_ctt::Parser::new(24, 80, 0);
    
    parser.process(b"Line 1\n");
    parser.process(b"Line 2\n");
    
    // Change screen size
    parser.screen_mut().set_size(30, 100);
    
    let screen = parser.screen();
    assert_eq!(screen.size(), (30, 100));
}

#[test]
fn test_alternate_buffer() {
    let mut parser = vt100_ctt::Parser::new(24, 80, 0);
    
    // Write to primary buffer
    parser.process(b"Primary buffer content\n");
    
    // Switch to alternate buffer
    parser.process(b"\x1b[?1049h");
    parser.process(b"Alternate buffer content\n");
    
    let screen = parser.screen();
    assert!(screen.contents().contains("Alternate"));
    
    // Switch back to primary buffer
    parser.process(b"\x1b[?1049l");
    
    let screen = parser.screen();
    assert!(screen.contents().contains("Primary"));
}

#[test]
fn test_scrollback() {
    let mut parser = vt100_ctt::Parser::new(3, 20, 10);
    
    // Fill screen and create scrollback - need enough lines to scroll
    for i in 1..=10 {
        parser.process(format!("Line {}\r\n", i).as_bytes());
    }
    
    let initial_contents = parser.screen().contents();
    
    // Try to scroll back - if scrollback data exists, content should change
    parser.screen_mut().set_scrollback(1);
    let scrolled_contents = parser.screen().contents();
    
    // If we have scrollback, the contents should be different after scrolling
    assert_ne!(initial_contents, scrolled_contents, "Should have scrollback data available");
}

#[test]
fn test_ratatui_integration() {
    // Test that the library works with ratatui feature (if enabled)
    #[cfg(feature = "tui-term")]
    {
        let mut parser = vt100_ctt::Parser::new(24, 80, 0);
        parser.process(b"Ratatui test\n");
        
        // Verify screen can be accessed
        let screen = parser.screen();
        assert!(screen.contents().contains("Ratatui test"));
    }
}
