//! Tools for dealing with YAML.

/// Utility for easy building of YAML mappings.
#[derive(Default)]
pub struct MapBuilder {
    /// The mapping being built.
    map: serde_yaml::Mapping,
}

impl MapBuilder {
    /// Add a new key-value pair to the mapping.
    pub fn set<K: Into<serde_yaml::Value>, V: Into<serde_yaml::Value>>(
        mut self,
        key: K,
        value: V,
    ) -> Self {
        self.map.insert(key.into(), value.into());
        self
    }

    /// Get the built mapping, as a YAML value.
    pub fn build(self) -> serde_yaml::Value {
        self.map.into()
    }
}

/// Extract and parse YAML front matter from a markdown file.
///
/// The front matter must be the first thing in the file other than whitespace,
/// or it will not be parsed. Front matter should begin with the line "---" and
/// end with the same, and contain only valid YAML.
pub fn parse(source: &str) -> Result<(serde_yaml::Value, String), serde_yaml::Error> {
    let (yaml, markdown) = Parser::parse(source);
    Ok((serde_yaml::from_str(&yaml)?, markdown))
}

/// The state of the parser.
enum State {
    /// The head (frontmatter) has not yet been opened.
    Start,
    /// The head has been opened but not closed.
    Head,
    /// The head has been closed or there was no head.
    Body,
}

/// A state-based parser to extract YAML front matter from a markdown file.
struct Parser {
    /// The current state of the parser.
    state: State,
    /// Frontmatter contents so-far parsed.
    head: String,
    /// Body contents so-far parsed.
    body: String,
}

impl Parser {
    /// Separate Markdown source into frontmatter and body.
    fn parse(source: &str) -> (String, String) {
        let mut parser = Self {
            state: State::Start,
            head: String::new(),
            body: String::new(),
        };
        for line in source.lines() {
            parser.push_line(line);
        }
        (parser.head, parser.body)
    }

    /// Parse the next line of source, changing the state as necessary.
    fn push_line(&mut self, line: &str) {
        self.state = match (&self.state, line.trim()) {
            (State::Start, "---") => State::Head,
            (State::Start, "") => State::Start,
            (State::Start | State::Body, _) => {
                self.body += line;
                self.body += "\n";
                // Anything other than a blank line before the head has started
                // means that there is no head.
                State::Body
            }
            (State::Head, "---") => State::Body,
            (State::Head, _) => {
                self.head += line;
                self.head += "\n";
                State::Head
            }
        }
    }
}
