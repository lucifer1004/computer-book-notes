macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn id<A>(a: A) -> A {
    a
}

fn main() {
    let f = |x| x + 4;
    let f_then_id = compose!(f, id);
    let id_then_f = compose!(id, f);
    assert_eq!(f_then_id(5), id_then_f(5));
}
