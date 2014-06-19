#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::Show;
use std::str;

pub trait Sym {
    fn to_str<'a>(&'a self) -> str::MaybeOwned<'a>;
}

pub enum Symbol<NonTerm/*:Symbol*/,Term/*:Symbol*/> { NT(NonTerm), T(Term), }

// FIXME: make priv when/if we have `impl Trait`
pub struct SententialForm_Symbols<'a, NT/*:Sym*/, T/*:Sym*/> {
    syms: &'a [Symbol<NT,T>],
    idx: uint,
}
pub trait SententialForm<NonTerm:Sym, Term:Sym> {
    fn symbols(&self) -> SententialForm_Symbols<NonTerm, Term>;
}

// FIXME: make priv when/if we have `impl Trait`
pub struct Sentence_Contents<'a, T/*:Sym*/> {
    terms: &'a [T],
    idx: uint,
}
pub trait Sentence<Term:Sym> {
    fn contents(&self) -> Sentence_Contents<Term>;
}

pub trait Language<NonTerm:Sym, Term:Sym> {
    
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

fn rule<LHS,RHS>(lhs: LHS, variants: Vec<RHS>) -> Rule<LHS,RHS> {
    Rule { lhs: lhs, variants: variants }
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

pub fn tdh() -> Grammar<Vec<&'static str>, Vec<&'static str>> {
    Grammar {
        start: vec!["Sentence"],
        rules: vec![rule(vec![            "Name"], vec![vec!["tom"],  vec!["dick"], vec!["harry"]]),
                    rule(vec![        "Sentence"], vec![vec!["Name"], vec!["List", "End"]]),
                    rule(vec![            "List"], vec![vec!["Name"], vec!["Name", ",", "End"]]),
                    rule(vec![",", "Name", "End"], vec![vec!["and", "Name"]])],
    }
}
