use crate::expression::Expression;

fn parenthesize(name: &str, exprs: &[&Expression]) -> String {
    let mut buf = String::new();
    buf.push('(');
    buf.push_str(name);
    for expr in exprs {
        buf.push(' ');
        buf.push_str(&get_expr_string(expr));
    }
    buf.push(')');
    buf
}

fn get_expr_string(expr: &Expression) -> String {
    match expr {
        Expression::Binary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]),
        Expression::Grouping(expr) => parenthesize("group", &[&(**expr).expr]),
        Expression::Literal(expr) => {
            format!("{:?}", expr.value)
        }
        Expression::Unary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.right]),
    }
}

//TODO: Currently does f all cause it doesn't know what to print due to the Any type. (make an enum for values)
pub fn print_ast(ast: Expression) {
    let str_rep = get_expr_string(&ast);
    println!("{}", str_rep);
}
