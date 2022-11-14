fn main() {
  // boolean
  let logical: bool = true;

  // Explicit type annotation
  let a_float: f64 = 1.0;
  let an_integer = 5i32

  // Infered type
  let default_float = 3.0;
  let default_integer = 7;

  // mut keyword is used to make a variable mutable
  let mut inferred_type = 12;
  inferred_type = 4294967296i64;

  // A mutable variable's value can be changed
  let mut mutable = 12;
  mutable = 21;

  // Error! The type of a variable can't be changed
  mutable = true;

  // Variables can be overwritten with shadowing
  let mutable = true;
}