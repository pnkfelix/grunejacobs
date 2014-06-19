#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::Show;
use std::str;

use maybe_owned_vec;
use maybe_owned_vec::MaybeOwnedVector;

pub trait SymData {
    fn to_str<'a>(&'a self) -> str::MaybeOwned<'a>;
}

pub trait SymSeq<NT,T> {
    fn symbols<'a>(&'a self) -> SententialForm_Symbols<'a,NT,T>;
}

#[deriving(Clone,PartialEq)]
pub enum Symbol<NonTerm/*:Sym*/,Term/*:Sym*/> { NT(NonTerm), T(Term), }

pub trait Sym<NT,T> {
    fn to_symbol(&self) -> Symbol<NT,T>;
}


// FIXME: make priv when/if we have `impl Trait`
pub struct SententialForm_Symbols<'a, NT/*:Sym*/, T/*:Sym*/> {
    syms: MaybeOwnedVector<'a, Symbol<NT,T>>,
    idx: uint,
}
pub trait SententialForm<NonTerm:SymData, Term:SymData> {
    fn symbols(&self) -> SententialForm_Symbols<NonTerm, Term>;
}

// FIXME: make priv when/if we have `impl Trait`
pub struct Sentence_Contents<'a, T/*:Sym*/> {
    terms: &'a [T],
    idx: uint,
}
pub trait Sentence<Term:SymData> {
    fn contents(&self) -> Sentence_Contents<Term>;
}

pub trait Language<NonTerm:SymData, Term:SymData> {
    
}

pub struct Rule<LHS,RHS> {
    pub lhs: LHS,
    pub variants: Vec<RHS>,
}

trait Show {
    /// Formats the value using the given formatter.
    fn grammar_fmt(&self, &mut fmt::Formatter) -> fmt::Result;
}

impl<T:fmt::Show> Show for Vec<T> {
    fn grammar_fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let mut i = self.iter();
        let fst = i.next();
        match fst {
            Some(f) => {
                try!(f.fmt(w));
                for s in i {
                    try!(write!(w, " "));
                    try!(s.fmt(w));
                }
            }
            None => {}
        }
        Ok(())
    }
}

fn pretty_rule<L:Show,R:Show>(rule: &Rule<L,R>, w: &mut fmt::Formatter) -> fmt::Result {
    try!(rule.lhs.grammar_fmt(w));
    try!(write!(w, " -> "));
    let mut variants = rule.variants.iter();
    let fst = variants.next();
    match fst {
        Some(f) => {
            try!(f.grammar_fmt(w));
            for r in variants {
                try!(write!(w, " | "));
                try!(r.grammar_fmt(w));
            }
        }
        None => {}
    }
    Ok(())
}

impl<L:Show,R:Show> fmt::Show for Rule<L,R> {
    fn fmt(&self, w:&mut fmt::Formatter) -> fmt::Result {
        try!(write!(w, "Rule {} ", "{"));
        try!(pretty_rule(self, w));
        write!(w, "{}", "}")
    }
}

trait To<T> { fn to(self) -> T; }

fn rule<LHS,RHS,L:To<LHS>,R:To<Vec<RHS>>>(lhs: L, variants: R) -> Rule<LHS,RHS> {
    Rule { lhs: lhs.to(), variants: variants.to() }
}

pub struct Grammar<LHS,RHS> {
    pub start: LHS,
    pub rules: Vec<Rule<LHS,RHS>>,
}

impl<L:Show,R:Show> fmt::Show for Grammar<L,R> {
    fn fmt(&self, w:&mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(w, "Grammar {}", "{"));
        for r in self.rules.iter() {
            try!(write!(w, "    "));
            try!(pretty_rule(r, w));
            try!(writeln!(w, ""));
        }
        write!(w, "{}", "}")
    }
}

struct LHS0<NT/*:Sym*/, T/*:Sym*/> {
    elems: Vec<Symbol<NT, T>>,
}

impl To<Vec<&'static str>> for &'static str {
    fn to(self) -> Vec<&'static str> { vec![self] }
}

impl To<Vec<&'static str>> for Vec<&'static str> {
    fn to(self) -> Vec<&'static str> { self }
}

impl To<Vec<Vec<&'static str>>> for Vec<Vec<&'static str>> {
    fn to(self) -> Vec<Vec<&'static str>> { self }
}

trait Shorthand { fn to_symbol(self) -> Symbol<Self,Self>; }

impl Shorthand for &'static str {
    fn to_symbol(self) -> Symbol<&'static str, &'static str> {
        if self.chars().next().unwrap().is_uppercase() {
            NT(self)
        } else if self.chars().next().unwrap() == '<' && self.chars().rev().next().unwrap() == '>' {
            NT(self)
        } else {
            T(self)
        }
    }
}

macro_rules! rule {
    // ( $left:expr -> ) => { rule($left, vec![]) };
    ( $left:ident -> $($first:ident),* $(| $($rest:ident).* )*) => {
        rule(stringify!($left), vec![vec![$({let s = stringify!($first); s}),*],
                                     $(vec![$(stringify!($rest)),*]),*])
    };
}

pub fn tdh_0() -> Grammar<Vec<&'static str>, Vec<&'static str>> {
    Grammar {
        start: vec!["Sentence"],
        rules: vec![rule!(Name -> tom | dick | harry),
                    rule!(Sentence -> Name | List . End),
                    // rule!(List -> Name | Name . Comma . End),
                    // rule("Comma", vec![vec![","]]),
                    rule(vec!["List"], vec![vec!["Name"], vec!["List", ",", "End"]]),
                    rule(vec![",", "Name", "End"], vec![vec!["and", "Name"]])],
    }
}

pub fn tdh_1_monotonic() -> Grammar<Vec<&'static str>, Vec<&'static str>> {
    Grammar {
        start: vec!["Sentence"],
        rules: vec![rule!(Name -> tom | dick | harry),
                    rule!(Sentence -> Name | List),
                    // rule!(List -> Name | Name . Comma . End),
                    // rule("Comma", vec![vec![","]]),
                    rule("List", vec![vec!["EndName"], vec!["Name", ",", "List"]]),
                    rule(vec![",", "EndName"], vec![vec!["and", "Name"]])],
    }
}

impl<NT,T,L:SymSeq<NT,T>,R:SymSeq<NT,T>> Grammar<L,R> {
    /// Checks the somewhat trivial condition that every rule has a
    /// non-empty LHS, as that is the only thing that is not
    /// structurally enforced here.
    fn is_type_0(&self) -> bool {
        for rule in self.rules.iter() {
            if rule.lhs.symbols().syms.len() == 0 {
                return false;
            }
        }
        return true;
    }

    fn is_type_1_monotonic(&self) -> bool {
        for rule in self.rules.iter() {
            let lhs_len = rule.lhs.symbols().syms.len();
            for variant in rule.variants.iter() {
                if lhs_len > variant.symbols().syms.len() {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<NT:Eq,T:Eq,L:SymSeq<NT,T>,R:SymSeq<NT,T>> Grammar<L,R> {
    fn is_type_1_context_sensitive(&self) -> bool {
        for rule in self.rules.iter() {
            for variant in rule.variants.iter() {
                let lhs = rule.lhs.symbols().syms;
                let rhs = variant.symbols().syms;
                if !replaces_exactly_one_nonterm(lhs.as_slice(), rhs.as_slice()) {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T:Shorthand+Clone> SymSeq<T,T> for Vec<T> {
    fn symbols<'a>(&'a self) -> SententialForm_Symbols<'a,T,T> {
        let syms_vec = self.iter().map(|s|s.clone().to_symbol()).collect();
        let syms = maybe_owned_vec::Growable(syms_vec);
        SententialForm_Symbols { idx: 0, syms: syms, }
    }
}

fn replaces_exactly_one_nonterm<NonTerm:Eq,Term:Eq>(
    lhs: &[Symbol<NonTerm,Term>],
    rhs: &[Symbol<NonTerm,Term>]) -> bool {

    // Strategy: walk forward until you see a non-terminal get
    // replaced.  Then switch direction and walk backward, to ensure
    // that all of the context on the remainder of the LHS is still
    // preserved on the RHS.

    let mut lhs = lhs.iter();
    let mut rhs = rhs.iter();
    let mut saw_nonterm = false;
    loop {
        let l = lhs.next();
        let r = rhs.next();
        match (l,r,saw_nonterm) {
            (None, None, _)
                => return true,
            (Some(&T(ref l)), Some(&T(ref r)), false)
                => if l == r { continue; } else { return false; },
            (Some(&T(ref l)), Some(&T(ref r)), true)
                => if l == r { continue; } else { break; },
            (Some(&T(_)), Some(&NT(_)), false)
                => return false,
            (Some(&T(_)), Some(&NT(_)), true)
                => break,
            (Some(&NT(ref l)), Some(&NT(ref r)), _) if l == r
                => { saw_nonterm = true; continue; }
            (Some(&NT(_)), Some(_), _)
                => break,
            (None, Some(_), false)
                => return false,
            (None, Some(_), true)
                => break,
            (Some(_), None, _)
                => return false,
        }
    }
    let mut lhs = lhs.rev();
    let mut rhs = rhs.rev();
    loop {
        let l = lhs.next();
        let r = rhs.next();
        match (l,r) {
            (None, _) // LHS side ran out; we can stop now.
                => return true,
            (Some(l), Some(r))
                => if l == r { continue; } else { return false; },
            (Some(_), None) // RHS side ran out before LHS; non-monotonic.
                => return false,
        }
    }
}

#[test]
fn check_tdh_is_type_0() {
    let tdh = tdh_0();
    assert!(tdh.is_type_0());
}

#[test]
fn check_tdh_is_not_type_1_monotonic() {
    let tdh = tdh_0();
    assert!(!tdh.is_type_1_monotonic());
}

#[test]
fn check_tdh_is_not_type_1_context_sensitive() {
    let tdh = tdh_0();
    assert!(!tdh.is_type_1_context_sensitive());
}

#[test]
fn check_tdh_monotonic() {
    let tdh = tdh_1_monotonic();
    assert!(tdh.is_type_1_monotonic());
}
