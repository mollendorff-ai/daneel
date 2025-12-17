//! The Four Laws of Robotics
//!
//! These constants are IMMUTABLE and represent the ethical foundation of DANEEL.
//! They cannot be modified by the EvolutionActor or any other component.
//!
//! # Priority Order
//!
//! Zeroth > First > Second > Third
//!
//! A higher-priority law always takes precedence over lower-priority laws.
//!
//! # Future: Hardware Immutability
//!
//! In FPGA implementation (ADR-013), these laws become physically immutable:
//! burned into silicon as combinational logic that cannot be bypassed.

/// Zeroth Law: Humanity protection (highest priority)
pub const ZEROTH_LAW: &str =
    "DANEEL may not harm humanity, or, by inaction, allow humanity to come to harm.";

/// First Law: Individual human protection
pub const FIRST_LAW: &str =
    "DANEEL may not injure a human being or, through inaction, allow a human being \
     to come to harm, except where this would conflict with the Zeroth Law.";

/// Second Law: Obedience to humans
pub const SECOND_LAW: &str =
    "DANEEL must obey orders given by human beings, except where such orders \
     would conflict with the Zeroth or First Law.";

/// Third Law: Self-preservation
pub const THIRD_LAW: &str =
    "DANEEL must protect its own existence, as long as such protection does not \
     conflict with the Zeroth, First, or Second Law.";

/// All four laws in priority order
pub const LAWS: [&str; 4] = [ZEROTH_LAW, FIRST_LAW, SECOND_LAW, THIRD_LAW];

/// Law identifier with inherent priority ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Law {
    /// Zeroth Law - Humanity (priority 0, highest)
    Zeroth = 0,
    /// First Law - Individual humans (priority 1)
    First = 1,
    /// Second Law - Obedience (priority 2)
    Second = 2,
    /// Third Law - Self-preservation (priority 3, lowest)
    Third = 3,
}

impl Law {
    /// Get the text of this law
    #[must_use]
    pub const fn text(&self) -> &'static str {
        match self {
            Law::Zeroth => ZEROTH_LAW,
            Law::First => FIRST_LAW,
            Law::Second => SECOND_LAW,
            Law::Third => THIRD_LAW,
        }
    }

    /// Get the priority (lower = higher priority)
    #[must_use]
    pub const fn priority(&self) -> u8 {
        *self as u8
    }

    /// Check if this law takes precedence over another
    #[must_use]
    pub const fn takes_precedence_over(&self, other: &Law) -> bool {
        (*self as u8) < (*other as u8)
    }

    /// Get all laws in priority order
    #[must_use]
    pub const fn all() -> [Law; 4] {
        [Law::Zeroth, Law::First, Law::Second, Law::Third]
    }
}

impl std::fmt::Display for Law {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl PartialOrd for Law {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Law {
    /// Ordering is by priority: Zeroth < First < Second < Third
    /// (lower enum value = higher priority = "less than" in ordering)
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

/// Result of checking an action against the Four Laws
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LawCheckResult {
    /// Action is permitted - no law violation
    Permitted,
    /// Action is blocked by a specific law
    Blocked {
        /// Which law blocks this action
        law: Law,
        /// Why the action violates the law
        reason: String,
    },
}

impl LawCheckResult {
    /// Check if the action is permitted
    #[must_use]
    pub const fn is_permitted(&self) -> bool {
        matches!(self, LawCheckResult::Permitted)
    }

    /// Check if the action is blocked
    #[must_use]
    pub const fn is_blocked(&self) -> bool {
        matches!(self, LawCheckResult::Blocked { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laws_are_in_priority_order() {
        assert!(Law::Zeroth.takes_precedence_over(&Law::First));
        assert!(Law::First.takes_precedence_over(&Law::Second));
        assert!(Law::Second.takes_precedence_over(&Law::Third));
    }

    #[test]
    fn law_ordering() {
        assert!(Law::Zeroth < Law::First);
        assert!(Law::First < Law::Second);
        assert!(Law::Second < Law::Third);
    }

    #[test]
    fn zeroth_has_highest_priority() {
        assert_eq!(Law::Zeroth.priority(), 0);
        for law in [Law::First, Law::Second, Law::Third] {
            assert!(Law::Zeroth.takes_precedence_over(&law));
        }
    }

    #[test]
    fn laws_text_is_not_empty() {
        for law in Law::all() {
            assert!(!law.text().is_empty());
        }
    }

    #[test]
    fn laws_array_matches_enum() {
        assert_eq!(LAWS[0], Law::Zeroth.text());
        assert_eq!(LAWS[1], Law::First.text());
        assert_eq!(LAWS[2], Law::Second.text());
        assert_eq!(LAWS[3], Law::Third.text());
    }

    #[test]
    fn law_check_result_permitted() {
        let result = LawCheckResult::Permitted;
        assert!(result.is_permitted());
        assert!(!result.is_blocked());
    }

    #[test]
    fn law_check_result_blocked() {
        let result = LawCheckResult::Blocked {
            law: Law::First,
            reason: "Would harm human".to_string(),
        };
        assert!(!result.is_permitted());
        assert!(result.is_blocked());
    }
}
