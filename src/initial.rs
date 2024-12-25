/*  =============  Expression/intial Domain Specific Language (EDSL) ============= */

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(i32),                   // Integer Literal
    Neg(Box<Expr>),             // negation
    Add(Box<Expr>, Box<Expr>)   // addition
}

// (...): everything inside of a parenthesis is marked as an expression and currently we have considered only negation and addition

impl Expr {
    pub fn lit(i: i32) -> Expr {
        Expr::Lit(i)
    }
    pub fn neg(r: Expr) -> Expr {
        Expr::Neg(Box::new(r))
    }
    pub fn add(r1: Expr, r2: Expr) -> Expr {
        Expr::Add(Box::new(r1), Box::new(r2))
    }
}

impl Expr {
    pub fn eval(&self) -> i32 { // recurcively evaluating the value
        match self {
            Expr::Lit(i) => *i,
            Expr::Add(a,b ) => a.eval() + b.eval(),
            Expr::Neg(a) => -a.eval()
        }
    }
    pub fn view(&self) -> String {
        match self {
            Expr::Lit(i) => format!("{}", i),
            Expr::Add(a, b ) => format!("({} + {})", a.view(), b.view()),
            Expr::Neg(a) => format!("(-{})", a.view())
        }
    }
}

/*
NOTE:

Adding new behaviors in the initial style corresponds to adding new interpreters of our language.
Easy! We can for example write a new function Expr.count that counts the number of sub-expressions, similar to Expr.view and Expr.eval.
    => because we can define impl even for a crate!! from outside scope!!

However, suppose we want to support multiplication in the EDSL.

Now we have to rewrite our enum to add a new variant and modify all existing interpreters to deal with the new variant.
If we can't access the source code for the enum or the interpreters, we're stuck. 
    => This we cannot do from outside scope!!
*/