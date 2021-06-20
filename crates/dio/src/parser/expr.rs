use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => p.bump(),
        _ => {}
    }

    let op = match p.peek() {
        Some(SyntaxKind::Plus) => Op::Add,
        Some(SyntaxKind::Minus) => Op::Sub,
        Some(SyntaxKind::Star) => Op::Mul,
        Some(SyntaxKind::Slash) => Op::Div,
        _ => return, // we’ll handle errors later.
    };
    p.bump();
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Ident@0..7 "counter""#]],
        );
    }
}
