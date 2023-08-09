use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    // tracing_subscriber::fmt::init();

    let sql = "select a a1,b, 123, myfunc(b), * \
    From data_source where a>b and b<100 and c BETWEEN 10 AND 20\
    ORDER BY a DESC, b limit 50 OFFSET 10";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}
