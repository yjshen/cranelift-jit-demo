extern crate core;

pub mod frontend;
pub mod jit;
mod assembler;
mod error;

#[cfg(test)]
mod tests {
    use crate::jit::JIT;
    use super::*;
    pub struct B(i8);

    pub struct A {
        ar: Vec<B>,
    }

    impl A {
        pub fn new() -> Self {
            Self {
                ar: vec![B(1), B(2), B(3)],
            }
        }
    }

    pub fn get_array(batch: &A, col_idx: usize) -> &B {
        &batch.ar[col_idx]
    }

    pub fn read_b(b: &B, col_idx: usize) {
        println!("{}", b.0 + col_idx as i8)
    }

    pub fn print(c: u64) {
        println!("{}", c)
    }

    #[test]
    fn a() {
        // Create the JIT instance, which manages all generated functions and data.
        let mut symbols = vec![];
        symbols.push(("get_array".to_owned(), get_array as *const u8));
        symbols.push(("read_b".to_owned(), read_b as *const u8));
        symbols.push(("print".to_owned(), print as *const u8));

        let mut jit = JIT::new(symbols);


        println!("the answer is: {}", run_foo(&mut jit).unwrap());
        let a = A::new();
        run_ccc(&mut jit, &a).unwrap();
        // println!(
        //     "recursive_fib(10) = {}",
        //     run_recursive_fib_code(&mut jit, 10)?
        // );
        println!(
            "iterative_fib(10) = {}",
            run_iterative_fib_code(&mut jit, 10).unwrap()
        );
        // run_hello(&mut jit)?;
    }

    fn run_foo(jit: &mut jit::JIT) -> Result<isize, String> {
        unsafe { run_code(jit, FOO_CODE, (1, 0)) }
    }

// fn run_recursive_fib_code(jit: &mut jit::JIT, input: isize) -> Result<isize, String> {
//     unsafe { run_code(jit, RECURSIVE_FIB_CODE, input) }
// }
//
fn run_iterative_fib_code(jit: &mut jit::JIT, input: isize) -> Result<isize, String> {
    unsafe { run_code(jit, ITERATIVE_FIB_CODE, input) }
}
//
// fn run_hello(jit: &mut jit::JIT) -> Result<isize, String> {
//     jit.create_data("hello_string", "hello world!\0".as_bytes().to_vec())?;
//     unsafe { run_code(jit, HELLO_CODE, ()) }
// }

    fn run_ccc(jit: &mut jit::JIT, a: &A) -> Result<(), String> {
        unsafe { run_code(jit, CCC, a) }
    }

    const CCC: &str = r#"
        fn x(a: ptr) {
            let v0: ptr = get_array(a, 0);
            read_b(v0, 0);
            let v1: ptr = get_array(a, 1);
            read_b(v1, 1);
            let v2: ptr = get_array(a, 2);
            read_b(v2, 2);
        }
    "#;

    /// Executes the given code using the cranelift JIT compiler.
    ///
    /// Feeds the given input into the JIT compiled function and returns the resulting output.
    ///
    /// # Safety
    ///
    /// This function is unsafe since it relies on the caller to provide it with the correct
    /// input and output types. Using incorrect types at this point may corrupt the program's state.
    unsafe fn run_code<I, O>(jit: &mut jit::JIT, code: &str, input: I) -> Result<O, String> {
        // Pass the string to the JIT, and it returns a raw pointer to machine code.
        let code_ptr = jit.compile(code)?;
        // Cast the raw pointer to a typed function pointer. This is unsafe, because
        // this is the critical point where you have to trust that the generated code
        // is safe to be called.
        let code_fn = core::mem::transmute::<_, fn(I) -> O>(code_ptr);
        // And now we can call it!
        Ok(code_fn(input))
        // Err("a".to_owned())
    }

    // A small test function.
//
// The `(c)` declares a return variable; the function returns whatever value
// it was assigned when the function exits. Note that there are multiple
// assignments, so the input is not in SSA form, but that's ok because
// Cranelift handles all the details of translating into SSA form itself.
const FOO_CODE: &str = r#"
fn foo(a: i64, b: i64) -> (c: i64){
    c = 0;
    c = c + 2;
}
"#;

// /// Another example: Recursive fibonacci.
// const RECURSIVE_FIB_CODE: &str = r#"
//     fn recursive_fib(n) -> (r) {
//         r: i64 = if n == 0 {
//                     0
//             } else {
//                 if n == 1 {
//                     1
//                 } else {
//                     recursive_fib(n - 1) + recursive_fib(n - 2)
//                 }
//             }
//     }
// "#;
//
// /// Another example: Iterative fibonacci.
const ITERATIVE_FIB_CODE: &str = r#"
    fn iterative_fib(n: i64) -> (r: i64) {
        if n == 0 {
            r = 0;
        } else {
            n = n - 1;
            let a: i64 = 0;
            r = 1;
            while n != 0 {
                let t: i64 = r;
                r = r + a;
                a = t;
                n = n - 1;
            }
        }
    }
"#;
//
// /// Let's say hello, by calling into libc. The puts function is resolved by
// /// dlsym to the libc function, and the string &hello_string is defined below.
// const HELLO_CODE: &str = r#"
// fn hello() -> (r) {
//     puts(&hello_string)
// }
// "#;


}

