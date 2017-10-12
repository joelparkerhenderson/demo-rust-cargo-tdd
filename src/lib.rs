#[cfg(test)]           // Annotation: use this in cargo test, not cargo build
mod tests {            // Define a module named "tests"
    use super::*;      // Bring in code from the outer module

    #[test]            // Annotation: the next function is a test
    fn foo_test() {    // Define a function as usual
      assert!(foo());  // Assert is a test macro that passes or panics
    }

}

pub fn foo() -> bool { // Define a public function named "foo" that returns a boolean 
    true               // Return true
}
