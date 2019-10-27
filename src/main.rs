use the_super_tiny_compiler_in_rust as comp;

fn main() {
    println!("Hello, compiled C code!");
    let tokens = comp::lexer::lexer(String::from("foo"));
    let lisp_ast = comp::parser::parser(tokens);
    let c_ast = comp::trans::transformer(lisp_ast);
}
