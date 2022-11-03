# Chapter 1. Category: The Essence of Composition

<!-- toc -->

```admonish note title="What makes a category?"
A category consists of objects and arrows that go between them.
```

## 1.1 Arrows as Functions

Arrows can be composed.

## 1.2 Properties of Composition

- Composition is associative.
- For every object there is a unit arrow (called *identity*).

## 1.3 Composition is the Essence of Programming

```admonish note title="What are the right chunks for composition?"
The information we need in order to compose chunks ("surface area") has to increase slower than the information we need in order to implement them ("volume").
```

```admonish quote
Category theory is extreme in the sense that it actively discourages us from looking inside the objects. An object in category theory is an abstract nebulous entity. All you can ever know about it is how it relates to other objects — how it connects with them using arrows.
```

## 1.4 Challenges

### Challenge 1.1 Implement identify

Identity function in Rust:

```rust
fn id<A>(a: A) -> A {
    a
}

fn main() {
    assert_eq!(id(1), 1);
    assert_eq!(id(1.0), 1.0);
    assert_eq!(id("1"), "1");
}
```

### Challenge 1.2 Implement composition

Composition function in Rust:

```rust
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

fn main() {
    let f = |x| x + 1;
    let g = |x| x * 2;
    let h = |x| x ^ 100;
    let fgh = compose!(f, g, h);
    assert_eq!(fgh(3), 108);
}
```

### Challenge 1.3 Validate composition

```rust
{{#rustdoc_include code/ch01/composition.rs:16:21}}
```

### Challenge 1.4 World-wide web

It depends on how we define the arrows whether the world-wide web can be considered a category. The web pages are objects without doubt. If we define the arrows as links, then `A->B` and `B->C` does not necessarily imply `A->C` because there might not be a direct link from `A` to `C`, and in this case the world-wide web is not a category. However, if we define the arrows as sequences of links, we can safely assume that `A->B` and `B->C` implies `A->C`, and thus the world-wide web can be a category.

See also Challenge 1.6.

### Challenge 1.5 Facebook

Facebook is not a category because friendship cannot be composed. Having `A->B` and `B->C` does not necessarily imply `A->C`.

### Challenge 1.6 Directed graph

A directed graph is a category if every node has a self link, and for every triplets `A, B, C`, if there is an edge `A->B` and an edge `B->C`, then there must be an edge `A->C`.
