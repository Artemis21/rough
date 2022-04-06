//! Defines a wrapper for `pulldown_cmark`'s parser (or any type implementing
//! the same iterator trait) which removes wrapped paragraph tags when they
//! only wrap a single image.
use pulldown_cmark::{Event, Tag};
use std::collections::VecDeque;

/// A wrapper for `pulldown_cmark`'s parser (or any type implementing the same
/// iterator trait) which removes wrapped paragraph tags when they only wrap a
/// single image.
pub struct InlineImages<'a, I: Iterator<Item = Event<'a>>> {
    /// The parser to get events from.
    parser: I,
    /// A buffer where events which have been requested from the underlying
    /// parser but not yet from whatever is consuming this wrapper are stored.
    buffer: VecDeque<Event<'a>>,
}

impl<'a, I: Iterator<Item = Event<'a>>> InlineImages<'a, I> {
    /// Wrap a parser in this converter. The wrapped parser can be used in the
    /// same way as the original one.
    pub fn new(parser: I) -> Self {
        InlineImages {
            parser,
            buffer: VecDeque::new(),
        }
    }

    /// Request events from the underlying parser until we reach an end image
    /// tag. The events will be stored in the buffer. Panics if we reach the
    /// end of the file without encountering an end image tag.
    fn buffer_until_image_end(&mut self) {
        loop {
            let event = self
                .parser
                .next()
                .expect("parser ended with unclosed image");
            if let Event::End(Tag::Image(..)) = event {
                self.buffer.push_back(event);
                break;
            }
            self.buffer.push_back(event);
        }
    }

    /// Handle the start of a paragraph, making the relevant transformations if
    /// the paragraph turns out to include only an image tag. Returns the next
    /// event to yield, but also always ends up with further events left in
    /// the buffer.
    fn on_paragraph_start(&mut self) -> Event<'a> {
        let event = self
            .parser
            .next()
            .expect("parser ended with unclosed paragraph");
        if let Event::Start(Tag::Image(..)) = event {
            self.buffer.push_back(event);
            self.buffer_until_image_end();
            let next = self
                .parser
                .next()
                .expect("parser ended with unclosed paragraph");
            if !matches!(next, Event::End(Tag::Paragraph)) {
                // The paragraph contains more than just the image, so re-insert
                // the paragraph start.
                self.buffer.push_front(Event::Start(Tag::Paragraph));
                self.buffer.push_back(next);
            }
            self.buffer.pop_front().expect("buffer not to be empty")
        } else {
            self.buffer.push_back(event);
            Event::Start(Tag::Paragraph)
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for InlineImages<'a, I> {
    type Item = Event<'a>;

    /// Get the next event, with the transformation applied.
    fn next(&mut self) -> Option<Event<'a>> {
        self.buffer.pop_front().or_else(|| {
            let event = self.parser.next()?;
            if event == Event::Start(Tag::Paragraph) {
                Some(self.on_paragraph_start())
            } else {
                Some(event)
            }
        })
    }
}
