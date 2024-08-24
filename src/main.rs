mod generator;
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
        html.example_class () {
            head () {
                title (My Site) {}
            }
            body () {
                h1 (Example Headline) {
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

    let generator = generator::Generator::new();
    let html = generator.generate_html(ast);
    println!("{}", html);

    // let lexer = Lexer::new(input).peekable();

    // for token in lexer {
    //     println!("{:?}", token);
    // }
}
