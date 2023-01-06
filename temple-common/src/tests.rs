use crate::syntax::lex::Lexer;
use crate::syntax::parse::Parser;

static TEMPLATE: &str = "
{%- let we_can_declar_vars = true; -%}
Can we declare vars? {{ we_can_declar_vars }}

{% for x in 0..1000 { %}
    x is currenly: {{ x }}
{%- } -%}
";

#[test]
fn test_lexer() {
    let tokens = Lexer::new(TEMPLATE).collect::<Vec<_>>();
    println!("{:?}", tokens)
}

#[test]
fn test_parser() {
    let nodes = Parser::new(TEMPLATE).parse_nodes().unwrap();
    println!("{:?}", nodes)
}
