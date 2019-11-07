extern crate mysql;

use mysql as my;
use std::io::BufWriter;
use rio_api::model::*;
use rio_turtle::NQuadsFormatter;
use rio_api::formatter::QuadsFormatter;

fn main() {
    let odbc_url = match std::env::var("MYSQL_URL") {
        Ok(url) => url,
        Err(_) => panic!("Error: Please specify MYSQL_URL environment variable."),
    };

    let mut conn: my::Conn = match my::Conn::new(odbc_url) {
        Ok(conn) => conn,
        Err(_) => panic!("Error: connectiing to the database"),
    };

    let mut stmt: my::Stmt = conn
        .prepare(
            r"
        SELECT
            N1.lex AS s_lex, N1.lang AS s_lang, N1.datatype AS s_datatype, N1.type AS s_type,
            N2.lex AS p_lex, N2.lang AS p_lang, N2.datatype AS p_datatype, N2.type AS p_type,
            N3.lex AS o_lex, N3.lang AS o_lang, N3.datatype AS o_datatype, N3.type AS o_type,
            N4.lex AS g_lex, N4.lang AS g_lang, N4.datatype AS g_datatype, N4.type AS g_type 
        FROM
            (SELECT g,s,p,o FROM Quads) Q
        LEFT OUTER JOIN Nodes AS N1 ON ( Q.s = N1.hash )
        LEFT OUTER JOIN Nodes AS N2 ON ( Q.p = N2.hash )
        LEFT OUTER JOIN Nodes AS N3 ON ( Q.o = N3.hash ) 
        LEFT OUTER JOIN Nodes AS N4 ON ( Q.g = N4.hash );
    ",
        )
        .unwrap();

    let mut formatter = NQuadsFormatter::new(BufWriter::new(std::io::stdout()));
    let mut result: my::QueryResult = stmt
        .execute(())
        .unwrap();
    let mut count = 0;

    while result.more_results_exists() {
        result.by_ref().for_each(|r| {
            count += 1;
            let mut row: my::Row = r.unwrap();

            let s_lex: String = row.take("s_lex").unwrap();
            let p_lex: String = row.take("p_lex").unwrap();
            let o_lex: String = row.take("o_lex").unwrap();
            let g_lex: String = row.take("g_lex").unwrap();

            let tmp: Option<String> = row.take("o_lang");

            let o_lang: Option<&str> = tmp.as_ref().map(|x| &**x);
            let o_datatype: String = row.take("o_datatype").unwrap();

            let q = Quad {
                subject: get_subject(
                    &s_lex,
                    row.take("s_lang").unwrap(),
                    row.take("s_datatype").unwrap(),
                    row.take("s_type").unwrap(),
                ),
                predicate: get_predicate(
                    &p_lex,
                    row.take("p_lang").unwrap(),
                    row.take("p_datatype").unwrap(),
                    row.take("p_type").unwrap(),
                ),
                object: get_obj(
                    &o_lex,
                    o_lang,
                    &o_datatype,
                    row.take("o_type").unwrap(),
                ),
                graph_name: get_graph(
                    &g_lex,
                    row.take("g_lang").unwrap(),
                    row.take("g_datatype").unwrap(),
                    row.take("g_type").unwrap(),
                ),
            };
            let _ = formatter.format(&q).expect("Error formatting triple.");
        });
    }
    formatter.finish();
}

fn get_subject<'a>(lex: &'a str, _lang: String, _datatype: String, typ: u32) -> NamedOrBlankNode {
    if typ == 1 {
        NamedOrBlankNode::BlankNode(BlankNode { id: lex })
    } else {
        NamedOrBlankNode::NamedNode(NamedNode { iri: lex })
    }
}

fn get_predicate<'a>(lex: &'a str, _lang: String, _datatype: String, _typ: u32) -> NamedNode {
    NamedNode { iri: &lex }
}

fn get_obj<'a>(lex: &'a str, lang: Option<&'a str>, datatype: &'a str, typ: u32) -> Term<'a> {
    match typ {
        1 => Term::BlankNode(BlankNode { id: lex }),
        2 => Term::NamedNode(NamedNode { iri: lex }),
        3 => {
            if lang.is_some() && !lang.unwrap().trim().is_empty() {
                Term::Literal(Literal::LanguageTaggedString { language: lang.unwrap(), value: lex })
            } else {
                Term::Literal(Literal::Simple { value: lex })
            }
        },
        _ => {
            let nn = NamedNode { iri: datatype };
            Term::Literal(Literal::Typed { datatype: nn, value: lex })
        },
    }
}

fn get_graph<'a>(lex: &'a str, _datatype: String, _lang: String, _typ: u32) -> Option<NamedOrBlankNode> {
    Some(NamedOrBlankNode::NamedNode(NamedNode { iri: lex }))
}
