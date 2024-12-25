use super::initial::*;

/*
For example, let's consider the case where we want to push down negation to literals, getting rid of double negation, like {-(-i)!! -> (i)}
You can think of this as an example of an optimization or rewriting pass.
*/
impl Expr {
    pub fn push_neg(self) -> Expr {
        match &self {
            Expr::Lit(_) => self,
            Expr::Add(r1, r2) => Expr::add(r1.to_owned().push_neg(), r2.to_owned().push_neg()),
            Expr::Neg(content) => match content.as_ref(){
                Expr::Lit(_) => self,
                Expr::Neg(r) => r.to_owned().push_neg(),
                Expr::Add(r1, r2) => Expr::add(
                    Expr::Neg(r1.clone()).push_neg(),
                    Expr::Neg(r2.clone()).push_neg(),
                ),
            }
        }
    }
}

