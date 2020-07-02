#[derive(Debug)]
/// Describes a rule match.
pub struct Match<'a> {
  len: usize,
  rewrite: &'a str
}

impl<'a> Match<'a> {
  pub(crate) fn new(len: usize, rewrite: &'a str) -> Match<'a> {
    Match { len, rewrite }
  }

  /// Return the length of the matched string.
  pub fn match_len(&self) -> usize {
    self.len
  }

  /// Return the string to replace the match with.
  pub fn rewrite(&self) -> &'a str {
    self.rewrite
  }
}

#[derive(Debug)]
/// L-system rule.
pub struct Rule {
  pattern: String,
  rewrite: String
}

impl Rule {
  /// Create a new `Rule`.
  pub fn new(pattern: String, rewrite: String) -> Rule {
    Rule { pattern, rewrite,}
  }

  /// Check whether the rule matches the supplied string, and if so, return the rewrite value.
  pub fn matches<'a>(&'a self, s: &str) -> Option<Match<'a>> {
    if s.starts_with(&self.pattern) {
      Some(Match::new(self.pattern.len(), &self.rewrite))
    } else {
      None
    }
  }
}