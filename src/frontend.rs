/// The AST node for our code generator.
pub enum Stmt {
    IfElse(Box<Expr>, Vec<Stmt>, Vec<Stmt>),
    WhileLoop(Box<Expr>, Vec<Stmt>),
    Declare(NameType),
    Assign(String, Box<Expr>),
    Initialization(NameType, Box<Expr>),
    SideEffect(Box<Expr>),
}

pub struct NameType {
    pub name: String,
    pub typ: Typ,
}

impl NameType {
    pub fn new(name: String, typ: Typ) -> Self {
        Self {
            name,
            typ,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Typ {
    Ptr,
    I64,
}

pub enum Expr {
    Literal(String),
    Identifier(String),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    GlobalDataAddr(String),
}

peg::parser!(pub grammar parser() for str {
    pub rule function() -> (String, Vec<NameType>, Option<NameType>, Vec<Stmt>)
        = _ "fn" _ name:identifier() _
        "(" params:((_ nt:name_typ() _ {nt}) ** ",") ")" _
        returns:( "->" _ "(" _ nt:name_typ() _ ")" _ {nt})?
        "{" _
        stmts:statements()
        _ "}" _
        { (name, params, returns, stmts) }

    rule statements() -> Vec<Stmt>
        = s:(statement()*) { s }

    rule statement() -> Stmt
        = if_else()
        / while_loop()
        / declare()
        / assignment()
        / initialization()
        / side_effect()

    rule if_else() -> Stmt
        = _ "if" _ e:expression() _ "{" _
        then_body:statements() _ "}" _ "else" _ "{" _
        else_body:statements() _ "}" _
        { Stmt::IfElse(Box::new(e), then_body, else_body) }

    rule while_loop() -> Stmt
        = _ "while" _ e:expression() _ "{" _
        loop_body:statements() _ "}" _
        { Stmt::WhileLoop(Box::new(e), loop_body) }

    rule declare() -> Stmt
        = _ "let" _ nt:name_typ() _ ";" _ { Stmt::Declare(nt) }

    rule assignment() -> Stmt
        = _ i:identifier() _ "=" _ e:expression() _ ";" _ { Stmt::Assign(i, Box::new(e)) }

    rule initialization() -> Stmt
        = _ "let" _ nt:name_typ() _ "=" _ e:expression() _ ";" _  { Stmt::Initialization(nt, Box::new(e)) }

    rule side_effect() -> Stmt
        = _ e:expression() _ ";" _ { Stmt::SideEffect(Box::new(e)) }

    rule expression() -> Expr
        = binary_op()

    rule name_typ() -> NameType
        = i:identifier() _ ":" _ t:typ() { NameType::new(i, t) }

    rule binary_op() -> Expr = precedence!{
        a:@ _ "==" _ b:(@) { Expr::Eq(Box::new(a), Box::new(b)) }
        a:@ _ "!=" _ b:(@) { Expr::Ne(Box::new(a), Box::new(b)) }
        a:@ _ "<"  _ b:(@) { Expr::Lt(Box::new(a), Box::new(b)) }
        a:@ _ "<=" _ b:(@) { Expr::Le(Box::new(a), Box::new(b)) }
        a:@ _ ">"  _ b:(@) { Expr::Gt(Box::new(a), Box::new(b)) }
        a:@ _ ">=" _ b:(@) { Expr::Ge(Box::new(a), Box::new(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Add(Box::new(a), Box::new(b)) }
        a:@ _ "-" _ b:(@) { Expr::Sub(Box::new(a), Box::new(b)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Mul(Box::new(a), Box::new(b)) }
        a:@ _ "/" _ b:(@) { Expr::Div(Box::new(a), Box::new(b)) }
        --
        i:identifier() _ "(" args:((_ e:expression() _ {e}) ** ",") ")" { Expr::Call(i, args) }
        i:identifier() { Expr::Identifier(i) }
        l:literal() { l }
    }

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule typ() -> Typ
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) {
            match n {
                "ptr" => Typ::Ptr,
                "i64" => Typ::I64,
                _ => unimplemented!()
            } } 
        }
        / expected!("type")

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }
        / "&" i:identifier() { Expr::GlobalDataAddr(i) }

    rule _() =  quiet!{[' ' | '\t' | '\n']*}
});
