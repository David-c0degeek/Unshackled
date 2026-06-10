//! Byte-level record framing for the stdio protocols.
//!
//! Records are LF-delimited. The framing contract is a hard requirement:
//! split on `\n` (0x0A) only, tolerate a trailing `\r` before the delimiter,
//! and never split on any other separator — Unicode line separators
//! (U+2028/U+2029) inside a record stay inside it. Splitting at the byte
//! level guarantees the Unicode rule structurally: no multi-byte UTF-8
//! sequence contains the 0x0A byte.

/// Incremental LF-record framer over raw bytes.
#[derive(Debug, Default)]
pub struct LineFraming {
    buf: Vec<u8>,
}

impl LineFraming {
    /// Feed a chunk; returns every complete record it finishes, as raw bytes
    /// without the LF (and without a trailing CR).
    pub fn push(&mut self, bytes: &[u8]) -> Vec<Vec<u8>> {
        self.buf.extend_from_slice(bytes);
        let mut records = Vec::new();
        while let Some(pos) = self.buf.iter().position(|&b| b == b'\n') {
            let mut record: Vec<u8> = self.buf.drain(..=pos).collect();
            record.pop(); // the LF
            if record.last() == Some(&b'\r') {
                record.pop();
            }
            records.push(record);
        }
        records
    }

    /// Any buffered bytes of an unterminated final record.
    #[must_use]
    pub fn remainder(&self) -> &[u8] {
        &self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(records: Vec<Vec<u8>>) -> Vec<String> {
        records
            .into_iter()
            .map(|r| String::from_utf8(r).unwrap())
            .collect()
    }

    #[test]
    fn splits_on_lf_only() {
        let mut framing = LineFraming::default();
        let records = strings(framing.push(b"{\"a\":1}\n{\"b\":2}\n"));
        assert_eq!(records, vec!["{\"a\":1}", "{\"b\":2}"]);
    }

    #[test]
    fn tolerates_trailing_cr() {
        let mut framing = LineFraming::default();
        let records = strings(framing.push(b"{\"a\":1}\r\n"));
        assert_eq!(records, vec!["{\"a\":1}"]);
    }

    #[test]
    fn a_cr_inside_a_record_is_preserved() {
        let mut framing = LineFraming::default();
        let records = strings(framing.push(b"{\"a\":\"x\ry\"}\n"));
        assert_eq!(records, vec!["{\"a\":\"x\ry\"}"]);
    }

    #[test]
    fn unicode_line_separators_stay_inside_the_record() {
        // U+2028 LINE SEPARATOR and U+2029 PARAGRAPH SEPARATOR are legal,
        // unescaped JSON string content; they must never split a record.
        let mut framing = LineFraming::default();
        let record = "{\"a\":\"x\u{2028}y\u{2029}z\"}\n";
        let records = strings(framing.push(record.as_bytes()));
        assert_eq!(records.len(), 1);
        assert!(records[0].contains('\u{2028}'));
        assert!(records[0].contains('\u{2029}'));
    }

    #[test]
    fn records_split_across_chunks_reassemble() {
        let mut framing = LineFraming::default();
        assert!(framing.push(b"{\"a\":").is_empty());
        assert!(framing.push(b"\"hello\"").is_empty());
        let records = strings(framing.push(b"}\n{\"b\":2}"));
        assert_eq!(records, vec!["{\"a\":\"hello\"}"]);
        assert_eq!(framing.remainder(), b"{\"b\":2}");
        let records = strings(framing.push(b"\n"));
        assert_eq!(records, vec!["{\"b\":2}"]);
    }

    #[test]
    fn multibyte_utf8_split_across_chunks_survives() {
        let mut framing = LineFraming::default();
        let line = "{\"a\":\"\u{65e5}\u{672c}\"}\n".as_bytes();
        let split = 8; // inside the first multi-byte character
        assert!(framing.push(&line[..split]).is_empty());
        let records = strings(framing.push(&line[split..]));
        assert_eq!(records, vec!["{\"a\":\"\u{65e5}\u{672c}\"}"]);
    }
}
