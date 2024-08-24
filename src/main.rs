mod lexer;
mod parser;

fn main() {
    let input = r#"
        html#test.example.hello {
            head {
                title {}
                meta {}
            }
            body {
                h1.test {
                    font-size: 16px;
                    font-size: red;
                }
            }
        }
    "#;

    let lexer = lexer::Lexer::new(input);
    for token in lexer {
        println!("{:?}", token);
    }

    let mut parser = parser::Parser::new(input);
    let ast = parser.parse();
    println!("{:?}", ast);
}
