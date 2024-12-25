mod initial;
mod r#final;
mod expression;
mod InitialPatternMatching;
mod finalPatternMatching;

use expression::tfm1;
use initial::*;
use r#final::*;
use InitialPatternMatching::*;
use finalPatternMatching::*;


/*
The final style certainly looks like a zero-cost abstraction,
while the initial style may not be negative or at least it'd need much more compiler optimizations to get rid of the enum variants and pattern matching.
*/


fn main() {
    { // using Initial Implementations
        let a = Expr::add(
        Expr::lit(8), 
        Expr::neg(Expr::add(Expr::lit(1), Expr::lit(2))));
        
        println!("initial.rs => a: {:?}", a);
        println!("initial.rs => a.view(): {}", a.view());
        
        let ans = a.eval();
        println!("initial.rs => a.eval(): {}", ans);
    }

    { // using Final Implementations
        let a = tf1::<Eval>();
        let v = tf1::<View>();

        println!("final.rs => a: {}", a);
        println!("final.rs => v: {}", v);

        // using tf2 which does come great magic of generic and associated types
        let c:i32 = tf2();
        let d:String = tf2(); // Based on the output expected it will retunr the results

        // OR: Will work just fine!!
        exprsym_eval(tf2());        
        exprsym_view(tf2());
    }

    { // using expression.rs Implementations
        let final_style: i32 = exprsym_eval(tfm1());
        assert_eq!(5, final_style);
        let v = exprsym_view(tfm1());
        println!("expression.rs => v: {}", v);
    }

    { // Pattern Matching in the initial.rs
        let a = Expr::add(
            Expr::lit(8), 
            Expr::neg(Expr::add(Expr::lit(1), Expr::lit(2)))
        );
        let s = Expr::neg(
            Expr::add(
            Expr::neg(Expr::lit(8)), 
            Expr::neg(Expr::add(Expr::neg(Expr::lit(1)), Expr::lit(2)))
        ));
        println!("patternmatching.rs => a: {}", a.view());
        println!("patternmatching.rs => s: {}", s.view());
        let b = a.push_neg();
        let t = s.push_neg();
        println!("patternmatching.rs => b: {}", b.view());
        println!("patternmatching.rs => t: {}", t.view());
            
    }

    { // Pattern Matching in final.rs
        let prev = exprsym_view(tf2());
        let now = exprsym_view(exprsym_push_neg(tf2()));
    }
}
