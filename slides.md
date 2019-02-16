## Rust std::fmt
--
seems so basic and common
stuff to learn
----------------------------------------------------------------------
##### My First Rust Program
```rust
fn main() {
  println!("Hello, world!");
}
```
----------------------------------------------------------------------
##### My Second Rust Program
```rust
fn main() {
  let name = "Ferris"
  println!("Hello, {}!", name);
}
```
----------------------------------------------------------------------
##### Why?
- curious - what's under the hood
- might learn something
- custom Debug
- impl Display
----------------------------------------------------------------------
##### println! ---> write!
```rust
println!("Hello, {}!", name);
```
```rust
write!(
  &mut io::stdout(),
  "{}",
  format_args!("Hello, {}!\n", name)
);
```
----------------------------------------------------------------------
##### write! -> write_fmt
```rust
&mut std::io::stdout().write_fmt(
  format_args!("{}" ,
    format_args!("Hello, {}!\n", name)
  )
)
```
----------------------------------------------------------------------
##### write_fmt = io::Write::write_fmt
```rust
fn write_fmt(
  &mut self,
  f: Arguments
) -> Result<()>
```
----------------------------------------------------------------------
#### How to create Arguments?
- no constructor <!-- .element: class="fragment" -->
- private fields <!-- .element: class="fragment" -->
----------------------------------------------------------------------
##### format_args!
```rust
format_args!(...) -> std:fmt::Arguments
```
----------------------------------------------------------------------
##### What be format_args!?
```rust
macro_rules! format_args {
  ($fmt:expr) => ({ /* compiler built-in */ });
  ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ });
}
```
<!-- .element: class="fragment small" -->
[src/std/macros.rs](https://doc.rust-lang.org/src/std/macros.rs.html)
----------------------------------------------------------------------
## Compiler Magic
----------------------------------------------------------------------
## Varargs
----------------------------------------------------------------------
### Format String Sub-Language
```ebnf
format := 
  | '{}'
  | '{' argument? ':' format_spec '}'
```
----------------------------------------------------------------------
format_spec configures a std::fmt::Formatter
----------------------------------------------------------------------
```ebnf
format_spec := 
  fill_align?
  sign?
  alternate?
  sign_aware_zero_pad?
  width?
  precision?
  type?
```
----------------------------------------------------------------------
```ebnf
fill_align := fill? align
  fill := char
  align :=
    | Left('<')
    | Center('^')
    | Right('>')
```
fill requires align, defaults to ' '
----------------------------------------------------------------------
```ebnf
sign := sign_plus | sign_minus
  sign_plus := Option<'+'>
  sign_minus := Option<'-'>
```
sign_minus is unused
----------------------------------------------------------------------
```ebnf
sign_aware_zero_pad := Option<'0'>
```
```rust
assert_eq!("00-1", format!("{:0>4}", -1));
assert_eq!("-001", format!("{:04}", -1));
```
<!-- .element: class="fragment" -->
----------------------------------------------------------------------
```ebnf
width := Option<usize | parameter>
```
```ebnf
precision :=
  '.' Option<usize | parameter | '*'>
```
----------------------------------------------------------------------
#### Does it compile?
```rust
fn main() {
  println!("{v:.2$} {0:0p$} {:.*} {}",
    2, 3.3, p = 4, v = 5.5);
}
```
```rust
5.5000 0002 3.30 4
```
<!-- .element: class="fragment" -->
----------------------------------------------------------------------
```ebnf
argument := integer | identifier
```
```ebnf
parameter := argument '$'
```
----------------------------------------------------------------------
```ebnf
type :=
  | ''  - Display
  | 'b' - Binary
  | 'o' - Octal
  | 'x' - LowerHex
  | 'X' - UpperHex
  | 'e' - LowerExp
  | 'E' - UpperExp
  | 'p' - Pointer
  | '?' | 'x?' | 'X?' - Debug
```
###### type specifies trait
----------------------------------------------------------------------
```rust
pub trait TYPE {
  fn fmt(
    &self,
    f: &mut Formatter
  ) -> Result<(), Error>;
}
```
#### requires UFCS
----------------------------------------------------------------------
##### Display
###### {} - default
###### also {:} <!-- .element: class="fragment small" -->
----------------------------------------------------------------------
##### {:#} alternate format
###### flag passed to trait impl
----------------------------------------------------------------------
###### alternates for integers
- {:#b} - 0b101010
- {:#o} - 0o52
- {:#x} - 0x2a
- {:#X} - 0x2A
----------------------------------------------------------------------
##### {:[eE]} - floats
- scientific notation
- no alternate format
----------------------------------------------------------------------
##### {:p} - pointer types
- lowercase hex with '0x' prefix
- alternate zero-pads to size_of
----------------------------------------------------------------------
## {:?} - Debug
----------------------------------------------------------------------
##### Debug with hex integers
- {:x?} - use lowercase hex
- {:X?} - use uppercase hex
- RFC #2226 (stable in 1.26.0)
----------------------------------------------------------------------
##### Debug with [boeEp]?
- hex case is common (e.g. &[u8])
- uses private kludge and deprecated API
- API unclear for others types
--
Avoids writing custom Debug impl
---------------------------------------------------------------------
#### Alternate Debug Format - {:#?}
- pretty print
- multi-line
----------------------------------------------------------------------
#### Custom Debug
```rust
impl fmt::Debug for MyType {
  fn fmt(
    &self,
    f: &mut fmt::Formatter
  ) -> Result<(), fmt::Error>
  { ... }
}
```
----------------------------------------------------------------------
#### Approach #0: f.write_str()
```rust
f.write_str("...")
```
- low-level
- cumbersome to produce anything non-trivial
----------------------------------------------------------------------
#### f.write_str (cont'd)
```rust
f.write_str("Foo (")?;
f.write_str(&42.to_string())?;
f.write_str(", ")?;
f.write_str("\"Hello World\"")?;
f.write_str(")")
```
```
Foo(42, "Hello World")
```
----------------------------------------------------------------------
#### Approach #1: f.write_fmt / write!
```rust
f.write_fmt(format!("...", self))
write!(f, "...", self...)
```
+ simple
- looses flags
- difficult to mimic std format
----------------------------------------------------------------------
#### f.write_fmt / write! (cont'd)
```rust
f.write_fmt(format_args!("Foo ({}, {:?})", 42, "Hello World"))
```
<!-- .element: class="small" -->
```rust
write!(f, "Foo ({}, {:?})", 42, "Hello World")
```
----------------------------------------------------------------------
#### Approach #2: f.debug_*()
- simple
- easy to reproduce std format
- preserves flags
- lower-level API still available
----------------------------------------------------------------------
##### DebugTuple
```rust
f.debug_tuple("Foo")
  .field(&42)
  .field(&"Hello World")
  .finish()
```
```
Foo(42, "Hello World")
```
----------------------------------------------------------------------
##### DebugStruct
```rust
f.debug_struct("Foo")
  .field("bar", &42)
  .field("baz", &"Hello World")
  .finish()
```
```
Foo { bar: 42, baz: "Hello World" }
```
----------------------------------------------------------------------
##### DebugList
```rust
let mut d = f.debug_list();
for i in &[42, 7] {
  d.entry(i);
}
d.finish()
```
```
[42, 7]
```
----------------------------------------------------------------------
##### DebugList
```rust
f.debug_list()
  .entries(self.0.iter())
  .finish()
```
----------------------------------------------------------------------
##### DebugSet
```rust
entry(...)
entries(self.0.iter())
```
```
{42, 7}
```
----------------------------------------------------------------------
##### DebugMap
```rust
entry(...);
entries(toTupleIterator(self));
```
```
{"A": 42, "B": 7}
```
----------------------------------------------------------------------
##### using format!
```rust
f.debug_struct(&format!("..."), ...)
  .field(&format!("..."), ...), &format!("..."), ...))
  .finish()
```
----------------------------------------------------------------------
#### other methods
```rust
fn pad(&mut self, s: &str);
fn pad_integral(
    &mut self, 
    is_nonnegative: bool, 
    prefix: &str, 
    buf: &str);
```
----------------------------------------------------------------------
#### where to use format strings
- `format!`
- `write!` / `writeln!`
- `print!` / `println!`
- `eprint!` / `eprintln!`
- `format_args!`
----------------------------------------------------------------------
### Caveats
----------------------------------------------------------------------
##### non-literal format string
```rust
let x = "{}";
println!(x, 42); 💥
```
```rust
const x: &str = "{}";
println!(x, 42); 💥
```
--
format string is compile time constant (macro)
----------------------------------------------------------------------
##### Debug and non-Debug generic
```rust
impl<T> Debug for T { ... }
impl<T: Debug> Debug for T { ... } 💥
```
--
no way (without specialization) to conditionally format based on whether type impls Debug
----------------------------------------------------------------------
#### debug output not necessarily parsable
----------------------------------------------------------------------
##### Debugging since 1.32
## dbg!
----------------------------------------------------------------------
##### shorthand for 
```rust
eprintln!(
  "[{}:{}] {} = {:#?}",
  file!(),
  line!(),
  stringify!($expr), $expr);
```
----------------------------------------------------------------------
##### dbg! example
```rust
let x = || 6;
let y = 7;
let z = dbg!(x() * y);
println("{}", z);
```
```rust
[src/main.rs:16] x() * y = 42
```
----------------------------------------------------------------------
##### nested dbg! example
```rust
let x = || 6;
let y = 7;
let z = dbg!(dbg!(x()) * y);
println!("{}", z);
```
```rust
[src/main.rs:16] x() = 6
[src/main.rs:16] dbg!(x (  )) * y = 42
```
----------------------------------------------------------------------
##### why?
- non-Debug field
- add info
- simplify format
----------------------------------------------------------------------
##### why?
- remove noise
  - wrapper types
  - large values
  - spaces, quotes, keys
----------------------------------------------------------------------
##### why?
- dereference "pointer"
- override via newtype
- format based on state
- enum
----------------------------------------------------------------------
# *** THE END ***
