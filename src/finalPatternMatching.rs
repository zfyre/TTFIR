use std::marker::PhantomData;

use super::r#final::*;
/*
Can we write push_neg in the final style, without pattern matching,
and produce a new expression that can then be further interpreted?
i.e. interpreted in Eval or View!! without explicitely adding Trait as MUl previously and then,
implementing it in both Eval and View!!

Since this push_neg has same kind of functionality for all the structs derieved from ExprSym
*/
enum Ctx {
    Pos,
    Neg,
}

struct CtxFun<TRepr>(Box<dyn Fn(&Ctx) -> TRepr>);

impl<TRepr> CtxFun<TRepr> {
    fn new(f: impl Fn(&Ctx) -> TRepr + 'static) -> Self {
        CtxFun(Box::new(f))
    }
}

// PhantomData here to get around "unconstrained type parameter T" in trait impl.
struct PushNeg<T>(PhantomData<T>);
impl<T: ExprSym + 'static> ExprSym for PushNeg<T> {
    type Repr = CtxFun<T::Repr>;
    type Repr2 = T::Repr2;

    fn lit(i: i32) -> Self::Repr {
        CtxFun::new(move |ctx| match ctx {
            Ctx::Pos => T::lit(i),
            Ctx::Neg => T::neg(T::lit(i)),
        })
    }

    fn neg(r: Self::Repr) -> Self::Repr {
        CtxFun::new(move |ctx| match ctx {
            Ctx::Pos => r.0(&Ctx::Neg),
            Ctx::Neg => r.0(&Ctx::Pos),
        })
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        CtxFun::new(move |ctx| T::add(r1.0(ctx), r2.0(ctx)))
    }
}

pub fn exprsym_push_neg<S: ExprSym>(e: &CtxFun<S::Repr>) -> S::Repr {
    e.0(&Ctx::Pos)
}