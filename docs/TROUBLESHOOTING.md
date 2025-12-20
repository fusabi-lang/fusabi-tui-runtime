# Troubleshooting Guide

Common issues and solutions for fusabi-tui-runtime.

## Build Issues

### Missing crossterm feature

**Error:**
```
error: cannot find type `CrosstermRenderer` in module `crossterm`
```

**Solution:** Enable the crossterm-backend feature:
```toml
[dependencies]
fusabi-tui-render = { version = "0.2", features = ["crossterm-backend"] }
```

### Conflicting Result types

**Error:**
```
error[E0277]: `?` couldn't convert the error to `RenderError`
```

**Solution:** The render prelude exports a custom `Result` type. Use fully qualified path:
```rust
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // ...
}
```

## Runtime Issues

### Terminal not restoring after crash

**Symptom:** Terminal stuck in raw mode, no cursor, can't see typed commands

**Solution:** Run `reset` or `stty sane` in your terminal

**Prevention:** Use proper cleanup in your code:
```rust
use std::panic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    // Set up panic hook for cleanup
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Your app code...

    // Normal cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
```

### Buffer size mismatch

**Error:**
```
RenderError::SizeMismatch { expected: Rect { ... }, actual: Rect { ... } }
```

**Cause:** Buffer created with different size than renderer expects

**Solution:** Always create buffer from renderer size:
```rust
let size = renderer.size()?;
let mut buffer = Buffer::new(size);
```

### Hot reload not detecting changes

**Symptom:** File changes aren't triggering reload

**Solutions:**
1. Check debounce timing (try longer delay):
   ```rust
   engine.enable_hot_reload_with_debounce(500)?; // 500ms
   ```
2. Ensure you're polling for changes:
   ```rust
   if let Some(changes) = engine.poll_changes() {
       if !changes.is_empty() {
           engine.reload()?;
       }
   }
   ```
3. Check file permissions
4. Some editors use atomic saves (write to temp, rename) - notify should handle this

## Widget Issues

### Widget not rendering

**Symptom:** Widget area appears empty

**Checklist:**
1. Is the area too small? Check width/height > 0
2. Are colors visible on your background?
3. Did you call `widget.render(area, &mut buffer)`?
4. Is the widget behind borders taking up all space?

**Example fix:**
```rust
// Wrong: Block takes entire area, leaves nothing for content
let block = Block::default().borders(Borders::ALL);
paragraph.block(block).render(area, &mut buffer);

// Right: Use inner area for content
let block = Block::default().borders(Borders::ALL);
let inner = block.inner(area);
block.render(area, &mut buffer);
paragraph.render(inner, &mut buffer);
```

### List selection not visible

**Symptom:** Selected item not highlighted

**Solution:** Ensure you're using StatefulWidget::render:
```rust
use fusabi_tui_widgets::widget::StatefulWidget;

let list = List::new(items).highlight_style(Style::new().bg(Color::Blue));
StatefulWidget::render(&list, area, &mut buffer, &mut list_state);
```

### Layout returning wrong sizes

**Symptom:** Constraints not being respected

**Check constraint math:**
```rust
// These don't add up to 100%
Layout::default()
    .constraints(&[
        Constraint::Percentage(50),
        Constraint::Percentage(60), // Overflow!
    ])

// Use Fill for flexible space
Layout::default()
    .constraints(&[
        Constraint::Length(3),
        Constraint::Fill(1),  // Takes remaining
        Constraint::Length(3),
    ])
```

## Scarab/Shared Memory Issues

### Shared memory connection failed

**Error:**
```
ScarabError::SharedMemory(...)
```

**Solutions:**
1. Ensure Scarab daemon is running
2. Check shared memory path matches: `/scarab_shm_v1`
3. Verify permissions on /dev/shm

### Rendering lag with Scarab

**Symptom:** Updates appear delayed

**Solutions:**
1. Reduce update frequency
2. Use differential updates (only changed cells)
3. Check sequence number synchronization

## Platform-Specific Issues

### macOS: Terminal flickering

**Cause:** Some terminal emulators have issues with rapid updates

**Solutions:**
1. Use iTerm2 or Alacritty
2. Reduce frame rate
3. Enable double buffering

### Windows: Colors look wrong

**Cause:** Windows Console has limited color support

**Solutions:**
1. Use Windows Terminal instead of cmd.exe
2. Use indexed colors (0-15) for compatibility
3. Enable virtual terminal processing

### Linux: Permission denied on /dev/shm

**Cause:** Shared memory requires write access

**Solution:**
```bash
sudo chmod 1777 /dev/shm
```

## Performance Issues

### High CPU usage

**Causes and solutions:**
1. **Too-fast render loop:** Add frame timing
   ```rust
   let frame_time = Duration::from_millis(16); // ~60 FPS
   std::thread::sleep(frame_time);
   ```
2. **Rendering unchanged content:** Check dirty flag
   ```rust
   if state.dirty {
       engine.render()?;
   }
   ```
3. **Full redraws:** Use differential rendering (built-in with CrosstermRenderer)

### Memory growth

**Cause:** Usually log/history buffers growing unbounded

**Solution:** Limit collection sizes:
```rust
self.log_items.push(message);
if self.log_items.len() > 1000 {
    self.log_items.remove(0);
}
```

## Debugging Tips

### Enable debug output

```rust
// Print buffer content for debugging
for y in 0..buffer.area.height {
    for x in 0..buffer.area.width {
        if let Some(cell) = buffer.get(x, y) {
            print!("{}", cell.symbol);
        }
    }
    println!();
}
```

### Use TestRenderer for testing

```rust
use fusabi_tui_render::test::TestRenderer;

let mut renderer = TestRenderer::new(80, 24);
// ... render widgets ...
renderer.assert_buffer(&expected_symbols);
```

### Log render timing

```rust
let start = std::time::Instant::now();
engine.render()?;
eprintln!("Render took: {:?}", start.elapsed());
```

## Getting More Help

1. **Search issues:** [GitHub Issues](https://github.com/fusabi-lang/fusabi-tui-runtime/issues)
2. **Ask a question:** Open a new issue with the "question" label
3. **Include details:**
   - Rust version (`rustc --version`)
   - OS and terminal emulator
   - Minimal reproduction code
   - Full error message
