/*!
Monitor back-end performance using Server-Timing the HTTP header.

```
use simple_server_timing_header::Timer;

fn handle_request() {
    let mut timer = Timer::new();
    // ... do some stuff
    timer.add("parse_headers");
    // ... do some more stuff
    timer.add("get_db_data");
    // Generate the header value
    assert_eq!(timer.header_value(), "parse_headers;dur=0, get_db_data;dur=0");
}
```
*/
use std::time::Instant;

/// Timer used to share performance metrics to the client using the HTTP Server-Timing header
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Server-Timing
pub struct Timer {
    last: Instant,
    timings: Vec<Timing>,
}

struct Timing {
    name: String,
    /// Time in milliseconds
    duration: u128,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            last: Instant::now(),
            timings: Vec::new(),
        }
    }

    /// Adds a named measurement, counting from the last one.
    /// Only alphanumeric characters are allowed, other characters are replaced with underscores.
    pub fn add(&mut self, name: &str) {
        let now = Instant::now();
        let duration = now.duration_since(self.last).as_millis();
        self.last = now;
        self.timings.push(Timing {
            name: name.into(),
            duration,
        });
    }

    // Header key for `Server-Timing`
    pub fn header_key() -> &'static str {
        "Server-Timing"
    }

    /// Returns the value for a Server-Timings header.
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Server-Timing
    pub fn header_value(&self) -> String {
        let mut out = String::new();
        use std::fmt::Write;
        for timing in self.timings.iter() {
            // Special characters and spaces are not properly parsed, so we replace them with underscores
            let name = timing.name.replace(|c: char| !c.is_alphanumeric(), "_");
            _ = write!(out, "{};dur={}, ", name, timing.duration);
        }
        // remove the trailing ", "
        out.pop();
        out.pop();

        out
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut timer = Timer::new();
        // ... do some stuff
        timer.add("parse headers");
        // ... do some more stuff
        timer.add("get_db_data");
        // Generate the header value
        assert_eq!(
            timer.header_value(),
            "parse_headers;dur=0, get_db_data;dur=0"
        );
    }
}
