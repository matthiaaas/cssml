mod ast;
mod lexer;
mod parser;

fn main() {
    let input = r#"
        html#test.example.hello () {
            head () {
                title {}
                meta {}
            }
            body (Hallo Welt) {
                h1 () {
                    font-size: 16px;
                    color: red;
                    font-weight: bold;

                    .test {
                        color: blue;
                    }
                }

                background: black;
            }

            background: red;
        }
    "#;

    // let lexer = lexer::Lexer::new(input);
    // for token in lexer {
    //     println!("{:?}", token);
    // }

    let mut parser = parser::Parser::new(input);
    let ast = parser.parse().ok().unwrap();
    println!("{:?}", ast);
    let output = ast.to_html().ok().unwrap();
    println!("{}", output);
}
