use std::str;

pub trait Sym {
    fn to_str<'a>(&'a self) -> str::MaybeOwned<'a>;
}

pub trait SententialForm<NonTerm:Sym, Term:Sym> {
    
}

pub trait Language<NonTerm:Sym, Term:Sym> {
    
}

pub struct Rule<LHS,RHS> {
    pub lhs: LHS,
    pub variants: Vec<RHS>,
}

pub struct Grammar<LHS,RHS> {
    pub start: LHS,
    pub rules: Vec<Rule<LHS,RHS>>,
}
