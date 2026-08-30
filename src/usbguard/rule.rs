// SPDX-License-Identifier: GPL-3.0-or-later

//! Parser for USBGuard rule strings.
//!
//! Both `listDevices` and `listRules` return rules as opaque strings, and the
//! device-presence signals carry one too, so parsing them is the only way to
//! learn anything about a device. The grammar is documented in
//! `usbguard-rules.conf(5)`; a device rule as emitted by the daemon looks like:
//!
//! ```text
//! allow id 1d6b:0002 serial "0000:00:14.0" name "xHCI Host Controller" \
//!     hash "jEP/6WzviqdJ5VSeTUY8PatCNBKeaREvo2OqdplND/o=" \
//!     parent-hash "..." via-port "usb1" with-interface 09:00:00 \
//!     with-connect-type "unknown"
//! ```
//!
//! Attribute values are either a bare word, a double-quoted string, or a set
//! written `{ a b c }` and optionally preceded by a set operator.

use std::fmt;

/// A single token of a rule string.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// A bare, unquoted word.
    Word(String),
    /// A double-quoted string, with escapes already resolved.
    Quoted(String),
    /// `{`
    SetOpen,
    /// `}`
    SetClose,
}

/// Set operators that may precede a `{ .. }` value.
const SET_OPERATORS: &[&str] = &["equals", "one-of", "none-of", "all-of", "any-of"];

/// Attribute names understood by USBGuard, in canonical output order.
const KNOWN_ATTRIBUTES: &[&str] = &[
    "id",
    "hash",
    "parent-hash",
    "name",
    "serial",
    "via-port",
    "with-interface",
    "with-connect-type",
    "label",
];

/// Failure to parse a rule string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The rule was empty or contained only whitespace.
    Empty,
    /// A double-quoted string was never closed.
    UnterminatedQuote,
    /// A `{` was never closed.
    UnterminatedSet,
    /// A `}` appeared with no matching `{`.
    UnexpectedSetClose,
    /// The leading token was not a valid rule target.
    BadTarget(String),
    /// An attribute name was given with no value.
    MissingValue(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty rule"),
            Self::UnterminatedQuote => f.write_str("unterminated quoted string"),
            Self::UnterminatedSet => f.write_str("unterminated `{` set"),
            Self::UnexpectedSetClose => f.write_str("unexpected `}`"),
            Self::BadTarget(t) => write!(f, "unknown rule target `{t}`"),
            Self::MissingValue(a) => write!(f, "attribute `{a}` has no value"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Split a rule string into tokens, honouring quotes, escapes and set braces.
fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
            }
            '{' => {
                chars.next();
                tokens.push(Token::SetOpen);
            }
            '}' => {
                chars.next();
                tokens.push(Token::SetClose);
            }
            '"' => {
                chars.next();
                let mut value = String::new();
                let mut closed = false;
                while let Some(c) = chars.next() {
                    match c {
                        '"' => {
                            closed = true;
                            break;
                        }
                        // USBGuard escapes `"` and `\` inside quoted strings.
                        '\\' => match chars.next() {
                            Some(escaped) => value.push(escaped),
                            None => return Err(ParseError::UnterminatedQuote),
                        },
                        other => value.push(other),
                    }
                }
                if !closed {
                    return Err(ParseError::UnterminatedQuote);
                }
                tokens.push(Token::Quoted(value));
            }
            _ => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == '{' || c == '}' || c == '"' {
                        break;
                    }
                    word.push(c);
                    chars.next();
                }
                tokens.push(Token::Word(word));
            }
        }
    }

    Ok(tokens)
}

/// The action a rule applies to a matching device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Target {
    /// Device is authorised.
    Allow,
    /// Device is present but de-authorised.
    Block,
    /// Device is de-authorised and removed from the system.
    Reject,
    /// Rule matches without changing authorisation (used by `match` rules).
    Match,
    /// Target reported by the daemon that this build does not know about.
    Unknown,
}

impl Target {
    /// Parse the textual form used in rule strings.
    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "allow" => Some(Self::Allow),
            "block" => Some(Self::Block),
            "reject" => Some(Self::Reject),
            "match" => Some(Self::Match),
            _ => None,
        }
    }

    /// The textual form used in rule strings.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Block => "block",
            Self::Reject => "reject",
            Self::Match => "match",
            Self::Unknown => "unknown",
        }
    }

    /// Convert from the integer used by the `Devices1` D-Bus interface.
    ///
    /// The numbering is USBGuard's `Rule::Target` enum: it is stable ABI for
    /// the D-Bus interface, so an unrecognised value means a newer daemon
    /// rather than a bug, and is surfaced as [`Target::Unknown`].
    pub fn from_dbus(value: u32) -> Self {
        match value {
            0 => Self::Allow,
            1 => Self::Block,
            2 => Self::Reject,
            3 => Self::Match,
            _ => Self::Unknown,
        }
    }

    /// Convert to the integer used by `applyDevicePolicy`.
    pub fn to_dbus(self) -> u32 {
        match self {
            Self::Allow => 0,
            Self::Block => 1,
            Self::Reject => 2,
            Self::Match => 3,
            Self::Unknown => u32::MAX,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.keyword())
    }
}

/// The value of a rule attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    /// A single value, e.g. `via-port "1-1"`.
    Single(String),
    /// A set, e.g. `with-interface one-of { 03:00:01 03:01:01 }`. The operator
    /// is `None` when the set was written bare, as `{ .. }`.
    Set {
        /// Set operator, if one was written.
        operator: Option<String>,
        /// Set members.
        values: Vec<String>,
    },
}

impl AttributeValue {
    /// The first (or only) value, if any.
    pub fn first(&self) -> Option<&str> {
        match self {
            Self::Single(v) => Some(v.as_str()),
            Self::Set { values, .. } => values.first().map(String::as_str),
        }
    }

    /// All values, in order.
    pub fn values(&self) -> Vec<&str> {
        match self {
            Self::Single(v) => vec![v.as_str()],
            Self::Set { values, .. } => values.iter().map(String::as_str).collect(),
        }
    }
}

/// A parsed USBGuard rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// The action the rule applies.
    pub target: Target,
    /// Attributes in the order they appeared, as `(name, value)` pairs.
    ///
    /// A `Vec` rather than a map because USBGuard permits a repeated
    /// attribute and because round-tripping should preserve order.
    pub attributes: Vec<(String, AttributeValue)>,
    /// The original, unmodified rule string.
    pub raw: String,
}

impl Rule {
    /// Parse a rule string as emitted by the USBGuard daemon.
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input)?;
        let mut iter = tokens.into_iter().peekable();

        let target = match iter.next() {
            None => return Err(ParseError::Empty),
            Some(Token::Word(w)) => Target::from_keyword(&w).ok_or(ParseError::BadTarget(w))?,
            Some(Token::Quoted(w)) => return Err(ParseError::BadTarget(w)),
            Some(Token::SetOpen) => return Err(ParseError::BadTarget("{".into())),
            Some(Token::SetClose) => return Err(ParseError::UnexpectedSetClose),
        };

        let mut attributes = Vec::new();

        while let Some(token) = iter.next() {
            let name = match token {
                Token::Word(w) => w,
                // A stray quoted string or brace where an attribute name was
                // expected is not something we can attribute meaning to; skip
                // it rather than failing the whole rule, since the important
                // fields may still parse.
                Token::SetClose => return Err(ParseError::UnexpectedSetClose),
                _ => continue,
            };

            // Conditions (`if <condition>`) are kept verbatim as a single
            // value; we do not need to interpret them.
            let mut operator = None;
            if let Some(Token::Word(w)) = iter.peek()
                && SET_OPERATORS.contains(&w.as_str())
            {
                operator = Some(w.clone());
                iter.next();
            }

            let value = match iter.peek() {
                Some(Token::SetOpen) => {
                    iter.next();
                    let mut values = Vec::new();
                    let mut closed = false;
                    for token in iter.by_ref() {
                        match token {
                            Token::SetClose => {
                                closed = true;
                                break;
                            }
                            Token::Word(w) | Token::Quoted(w) => values.push(w),
                            Token::SetOpen => return Err(ParseError::UnterminatedSet),
                        }
                    }
                    if !closed {
                        return Err(ParseError::UnterminatedSet);
                    }
                    AttributeValue::Set { operator, values }
                }
                Some(Token::Word(_) | Token::Quoted(_)) => match iter.next() {
                    Some(Token::Word(w) | Token::Quoted(w)) => AttributeValue::Single(w),
                    _ => unreachable!("peeked a value token"),
                },
                // An attribute name with no value at all. `label` and friends
                // always carry one, so treat this as malformed.
                _ => return Err(ParseError::MissingValue(name)),
            };

            attributes.push((name, value));
        }

        Ok(Self {
            target,
            attributes,
            raw: input.to_string(),
        })
    }

    /// The first value of the named attribute, if present.
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, v)| v.first())
    }

    /// All values of the named attribute, across repeats.
    pub fn attribute_values(&self, name: &str) -> Vec<&str> {
        self.attributes
            .iter()
            .filter(|(n, _)| n == name)
            .flat_map(|(_, v)| v.values())
            .collect()
    }

    /// Whether the attribute name is one USBGuard defines.
    pub fn is_known_attribute(name: &str) -> bool {
        KNOWN_ATTRIBUTES.contains(&name)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

/// Quote and escape a value for inclusion in a rule string.
///
/// USBGuard's parser treats `"` and `\` as special inside a quoted string, so
/// both must be escaped. Everything else is passed through.
pub fn quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        if c == '"' || c == '\\' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A real rule string as emitted by `usbguard list-devices` on a laptop.
    const DEVICE_RULE: &str = concat!(
        r#"allow id 1d6b:0002 serial "0000:00:14.0" name "xHCI Host Controller" "#,
        r#"hash "jEP/6WzviqdJ5VSeTUY8PatCNBKeaREvo2OqdplND/o=" "#,
        r#"parent-hash "kv3Xt5uHmLLbHkfDbNbUmDLNJVEJvxWn0Zqcz3XZSjA=" "#,
        r#"via-port "usb1" with-interface 09:00:00 with-connect-type "unknown""#
    );

    #[test]
    fn parses_a_real_device_rule() {
        let rule = Rule::parse(DEVICE_RULE).expect("should parse");
        assert_eq!(rule.target, Target::Allow);
        assert_eq!(rule.attribute("id"), Some("1d6b:0002"));
        assert_eq!(rule.attribute("name"), Some("xHCI Host Controller"));
        assert_eq!(rule.attribute("serial"), Some("0000:00:14.0"));
        assert_eq!(rule.attribute("via-port"), Some("usb1"));
        assert_eq!(rule.attribute("with-interface"), Some("09:00:00"));
        assert_eq!(rule.attribute("with-connect-type"), Some("unknown"));
        assert_eq!(
            rule.attribute("hash"),
            Some("jEP/6WzviqdJ5VSeTUY8PatCNBKeaREvo2OqdplND/o=")
        );
        assert_eq!(rule.raw, DEVICE_RULE);
    }

    #[test]
    fn parses_empty_quoted_values() {
        let rule = Rule::parse(r#"block id 8087:0024 serial "" name """#).unwrap();
        assert_eq!(rule.target, Target::Block);
        assert_eq!(rule.attribute("serial"), Some(""));
        assert_eq!(rule.attribute("name"), Some(""));
    }

    #[test]
    fn parses_bare_interface_set() {
        let rule = Rule::parse("allow id 046d:c52b with-interface { 03:01:01 03:01:02 03:00:00 }")
            .unwrap();
        let values = rule.attribute_values("with-interface");
        assert_eq!(values, vec!["03:01:01", "03:01:02", "03:00:00"]);
    }

    #[test]
    fn parses_set_with_operator() {
        let rule = Rule::parse("allow with-interface one-of { 03:00:01 03:01:01 }").unwrap();
        let (_, value) = &rule.attributes[0];
        assert_eq!(
            value,
            &AttributeValue::Set {
                operator: Some("one-of".into()),
                values: vec!["03:00:01".into(), "03:01:01".into()],
            }
        );
    }

    #[test]
    fn parses_escaped_quotes_in_names() {
        // A device whose name legitimately contains a double quote.
        let rule = Rule::parse(r#"allow id 1234:5678 name "13\" Display""#).unwrap();
        assert_eq!(rule.attribute("name"), Some(r#"13" Display"#));
    }

    #[test]
    fn parses_backslash_in_names() {
        let rule = Rule::parse(r#"allow name "back\\slash""#).unwrap();
        assert_eq!(rule.attribute("name"), Some(r"back\slash"));
    }

    #[test]
    fn rejects_unknown_target() {
        assert_eq!(
            Rule::parse("permit id 1234:5678"),
            Err(ParseError::BadTarget("permit".into()))
        );
    }

    #[test]
    fn rejects_empty_rule() {
        assert_eq!(Rule::parse("   "), Err(ParseError::Empty));
    }

    #[test]
    fn rejects_unterminated_quote() {
        assert_eq!(
            Rule::parse(r#"allow name "unterminated"#),
            Err(ParseError::UnterminatedQuote)
        );
    }

    #[test]
    fn rejects_unterminated_set() {
        assert_eq!(
            Rule::parse("allow with-interface { 03:00:00"),
            Err(ParseError::UnterminatedSet)
        );
    }

    #[test]
    fn quote_escapes_specials() {
        assert_eq!(quote("plain"), r#""plain""#);
        assert_eq!(quote(r#"a"b"#), r#""a\"b""#);
        assert_eq!(quote(r"a\b"), r#""a\\b""#);
    }

    #[test]
    fn quoted_values_round_trip_through_the_parser() {
        // Anything `quote` produces must parse back to the original value,
        // otherwise a rule we build could match the wrong device.
        for original in [
            "plain",
            "",
            r#"has "quotes""#,
            r"has\backslash",
            "spaces and { braces }",
        ] {
            let rule_text = format!("allow name {}", quote(original));
            let rule = Rule::parse(&rule_text).expect("built rule should parse");
            assert_eq!(rule.attribute("name"), Some(original), "for {original:?}");
        }
    }

    #[test]
    fn dbus_targets_round_trip() {
        for target in [Target::Allow, Target::Block, Target::Reject, Target::Match] {
            assert_eq!(Target::from_dbus(target.to_dbus()), target);
        }
        assert_eq!(Target::from_dbus(99), Target::Unknown);
    }
}
