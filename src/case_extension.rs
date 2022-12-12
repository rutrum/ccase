use convert_case::{Case, Casing, Pattern};

/// Extensions Case with additional functions.
pub trait CaseExtension {

    /// The name of the case in the case
    fn name_in_case(self) -> String;

    /// If case is an alias to another (one directional)
    fn is_alias(&self) -> Option<Case>;

    /// Alternative shorter name
    fn short_name(&self) -> Option<&str>;

    /// Converts case name to flat case
    fn name_to_flat_case(&self) -> String;
}

impl CaseExtension for Case {
    fn name_in_case(self) -> String {
        format!("{:?}Case", self).to_case(self)
    }

    fn is_alias(&self) -> Option<Case> {
        use Case::*;
        match self {
            UpperCamel => Some(Pascal),
            UpperKebab => Some(Cobol),
            ScreamingSnake => Some(UpperSnake),
            _ => None,
        }
    }

    fn short_name(&self) -> Option<&'static str> {
        use Case::*;
        match self {
            PseudoRandom => Some("pseudo"),
            ScreamingSnake => Some("screaming"),
            Alternating => Some("alternate"),
            _ => None,
        }
    }

    fn name_to_flat_case(&self) -> String {
        format!("{:?}", self).to_case(Case::Flat)
    }
}

pub trait PatternExtension {

    /// Prints an example of pattern
    fn example(&self) -> String;
}

impl PatternExtension for Pattern {
    fn example(&self) -> String {
        use Pattern::*;
        match self {
            Lowercase => "lower, lower, ...",
            Uppercase => "UPPER, UPPER, ...",
            Capital => "Capital, Capital, ...",
            Sentence => "Capital, lower, lower, ...",
            Camel => "lower, Capital, Capital, ...",
            Alternating => "aLtErNaTiNg, aLtErNaTiNg, ...",
            Toggle => "tOGGLE, tOGGLE, ...",
            PseudoRandom => "pSUeDorANdOm, pSUedORaNdoM, ...",
            Random => "RanDOM, RAndom, ...",
        }.to_string()
    }
}
