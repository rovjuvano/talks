```rust
trait MyTrait<A> {
    type B;
    fn method<C>(&self, arg1: A, arg2: Self::B, arg3: C) -> (A, Self::B, C);
}
struct MyType<T> { ... }
impl<T, U> MyTrait<T> for MyType<U> {
    type B = U;
    fn method<V>(&self, _arg1: T, _arg2: Self::B, _arg3: V) -> (T, U, V) {
        unimplemented!()
    }
}
```

| use            | no traits  | A: trait      | B: assoc. type | C: function    |
|----------------|------------|---------------|----------------|----------------|
| type at let    | inferred   | placeholder   | inferred       | placeholder    |
| turbofish      | no         | no            | no             | yes            |
| generic impl   | no         | yes           | yes - E0207    | yes            |
| multiple impls | 1/per type | many/type     | 1/per type     | 1 generic/type |
