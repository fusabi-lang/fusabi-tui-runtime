#!/bin/bash
# Temporary test script to verify our widget implementation

cd /home/beengud/fusabi-lang/fusabi-tui-runtime/crates/fusabi-tui-widgets

# Create a temporary lib.rs with only our modules
cat > src/lib_test.rs << 'EOF'
#![allow(dead_code)]

pub mod borders;
pub mod block;
pub mod widget;

pub use block::{Block, Padding, Title, TitleAlignment, TitlePosition};
pub use borders::{BorderType, Borders};
pub use widget::{StatefulWidget, Widget};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use fusabi_tui_core::{buffer::Buffer, layout::Rect, style::{Color, Style}};

    #[test]
    fn test_full_integration() {
        let block = Block::default()
            .title("Test")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding::uniform(1));

        let area = Rect::new(0, 0, 20, 10);
        let mut buffer = Buffer::new(area);

        block.render(area, &mut buffer);

        // Verify rendering worked
        assert_eq!(buffer.get(0, 0).unwrap().symbol, "╭");
        assert_eq!(buffer.get(19, 0).unwrap().symbol, "╮");

        // Verify inner area calculation
        let inner = block.inner(area);
        assert_eq!(inner, Rect::new(2, 2, 16, 6));
    }
}
EOF

# Compile and test just our modules
rustc --test --edition 2021 --crate-type lib src/lib_test.rs \
    --extern fusabi_tui_core=/home/beengud/.cargo/target/debug/deps/libfusabi_tui_core-*.rlib \
    --extern bitflags=/home/beengud/.cargo/target/debug/deps/libbitflags-*.rlib \
    -L /home/beengud/.cargo/target/debug/deps \
    -o test_our_widgets 2>&1

if [ $? -eq 0 ]; then
    echo "✓ Compilation successful"
    ./test_our_widgets
    rm -f test_our_widgets src/lib_test.rs
else
    echo "✗ Compilation failed"
    rm -f src/lib_test.rs
fi
