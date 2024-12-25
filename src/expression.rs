
/* =============== EXTENDING ORIGINAL final.rs INTERPRETER WITHOUT MAKING CHANGES IN IT ALREADY!! ================= */

use super::r#final::*;


// extending the ExprSym trait & defining MulExprSym as a subtrait of ExprSum
pub trait MulExprSym: ExprSym {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}

impl MulExprSym for Eval {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        r1*r2
    }
}
impl  MulExprSym for View {
    fn mul(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        format!("({} * {})", r1, r2)
    }
}

pub fn tfm1<E: MulExprSym<Repr = T>, T: HasExprSym<ES = E>>() -> T {
    E::add(E::lit(7), E::neg(E::mul(E::lit(1), E::lit(2))))
}   