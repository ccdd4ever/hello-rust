use sqlparser::dialect::Dialect;


#[derive(Debug, Default)]
pub struct MyDialect;

impl Dialect for MyDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..'z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || ['_', ':', '/', '&', '=', '-', '.', '?'].contains(&ch)
    }
}

pub fn example_sql() -> String {
    let url = "https://raw.github.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!("SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
    FROM {} WHERE new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5", url);
    println!("{}", sql);
    sql
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlparser::parser::Parser;
    use tracing::Level;

    #[test]
    fn it_works() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();

        assert!(Parser::parse_sql(&MyDialect::default(), &example_sql()).is_ok());
    }
}