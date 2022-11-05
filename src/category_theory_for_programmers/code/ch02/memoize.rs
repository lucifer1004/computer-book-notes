use std::collections::HashMap;
use std::hash::Hash;

fn memoize<A, B, F>(f: F) -> impl FnMut(A) -> B
where
    F: Fn(A) -> B,
    A: Eq + Hash + Clone,
    B: Clone,
{
    let mut cache: HashMap<A, B> = HashMap::new();
    move |arg| {
        if let Some(result) = cache.get(&arg) {
            result.clone()
        } else {
            println!("Evaluated!");
            let result = f(arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}

fn main() {
    let mut f = memoize(|n| 2 * n);
    println!("{}", f(20)); // First-time call, evaluated
    println!("{}", f(20)); // Memoization used
}
