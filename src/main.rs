mod lexer;
mod parser;

fn main() {
    // let input = r#"
    //     html {
    //       head {
    //         title (My Website) {}
    //       }

    //       body {
    //         h1 (Welcome to my website) {
    //           font-size: 40px;
    //           color: red;
    //         }

    //         p.lead.heading_paragraph (This is a paragraph) {
    //           font-size: 20px;
    //         }

    //         div#logo () {
    //           width: 100%;
    //           margin: 0 auto;
    //         }

    //         .lead {
    //           color: blue;
    //         }
    //       }
    //     }
    // "#;
    //

    let input = r#"
        html {
            head {
                title {}
                meta {}
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

    // let lexer = Lexer::new(input).peekable();

    // for token in lexer {
    //     println!("{:?}", token);
    // }
}
