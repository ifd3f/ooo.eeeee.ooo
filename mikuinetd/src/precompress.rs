use std::{cell::RefCell, io::{BufRead, Read, Write}, rc::Rc};

use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

/// A [`Vec<u8>`] wrapped in a [`Rc`] and [`RefCell`]. Basically, this lets us
/// pass a handle to [`Terminal`] while at the same time being able to inspect
/// its outputted results.
#[derive(Clone, Default)]
struct SharedBuf {
    inner: Rc<RefCell<Vec<u8>>>,
}

impl SharedBuf {
    /// Take the buffer inside and replace it with an empty Vec.
    fn take_and_clear(&self) -> Vec<u8> {
        std::mem::take(&mut *self.inner.borrow_mut())
    }
}

impl Write for SharedBuf {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.borrow_mut().flush()
    }
}

/// Given a list of strings with newlines, use ratatui to render it to a
/// stream of terminal control characters that, when outputted sequentially,
/// will render that terminal.
pub fn precompress(frames: impl Iterator<Item = String>) -> impl Iterator<Item = Vec<u8>> {
    let buf = SharedBuf::default();
    let mut terminal = Terminal::new(CrosstermBackend::new(buf.clone())).unwrap();

    frames.map(move |afr| {
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(&*afr), f.size());
        }).unwrap();
        buf.take_and_clear()
    })
}
