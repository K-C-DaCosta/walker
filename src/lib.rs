#[derive(Copy,Clone)]
enum Ast<'a>{
    Add{lhs:&'a Ast<'a>,rhs:&'a Ast<'a>},
    Sub{lhs:&'a Ast<'a>,rhs:&'a Ast<'a>},
    Mul{lhs:&'a Ast<'a>,rhs:&'a Ast<'a>},
    Div{lhs:&'a Ast<'a>,rhs:&'a Ast<'a>},
}