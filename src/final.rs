
pub trait ExprSym {
    type Repr;
    type Repr2; // Note that we can have more than one assocaited types

    fn lit(i: i32) -> Self::Repr;
    fn neg(r: Self::Repr) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;
}

pub struct Eval {}
pub struct View {}

impl ExprSym for Eval {
    type Repr = i32;
    type Repr2 = ();
    fn lit(i: i32) -> Self::Repr {
        i
    }
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        r1 + r2
    }
    fn neg(r: Self::Repr) -> Self::Repr {
        -r
    }
}
impl ExprSym for View {
    type Repr = String;
    type Repr2 = ();
    fn lit(i: i32) -> Self::Repr {
        i.to_string()
    }
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        format!("({} + {})", r1, r2)
    }
    fn neg(r: Self::Repr) -> Self::Repr {
        format!("(-{})", r)
    }
}

pub fn tf1<E>() -> E::Repr // this function will be accessed by anyfunction that implements ExprSym
where E: ExprSym {
    E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
}


// https://getcode.substack.com/p/efficient-extensible-expressive-typed

pub fn exprsym_eval(e: i32) -> i32 { // Since rust compiler does'nt know about E::Repr is from which struct
    e
}
pub fn exprsym_view(e: String) -> String {
    e
}

// we will map the struct type from return type to input
// i.e. map from E::Repr to E

pub trait HasExprSym {
    type ES: ExprSym; // Here we make ES as an alias of ExprSym, it's an associated type at the same time of the trait
}

// i32 -> Eval
impl HasExprSym for i32 {
    type ES = Eval;
}
// String -> View
impl HasExprSym for String {
    type ES = View;
}

pub fn tf2<E: ExprSym<Repr = T>, T: HasExprSym<ES = E>>() -> T {
    E::add(E::lit(8), E::neg(E::add(E::lit(1), E::lit(2))))
}
/* NOTE: The usefulness of this trick is debatable. It barely scales up to higher-order problems in part 2, and has limits. */
