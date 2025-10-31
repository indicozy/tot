use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Қт" => "Err",
        "Жарайды" => "Ok",
        "Жол" => "String",
        "ХэшМап" => "HashMap",
        "Әдепкі" => "Default",
        "Қате" => "Error",
        "Опция" => "Option",
        "Кейбір" => "Some",
        "Жоқ" => "None",
        "Нәтиже" => "Result",
        "Өзім" => "Self",
        "басыпшж" => "println",
        "үзіліс" => "break",
        "асинх" => "async",
        "күту" => "await",
        "цикл" => "loop",
        "көшу" => "move",
        "жәшік" => "crate",
        "қолжетімсіз_код" => "unreachable_code",
        "ретінде" => "as",
        "тұрақты" => "const",
        "қасиет" => "trait",
        "қауіпсіземес" => "unsafe",
        "ішінде" => "in",
        "дан" => "from",
        "дин" => "dyn",
        "ашу" => "unwrap",
        "әдепкі" => "default",
        "сіл_ретінде" => "as_ref",
        "еш" => "io",
        "сыртқы" => "extern",
        "жалған" => "false",
        "фн" => "fn",
        "супер" => "super",
        "кірістіру" => "insert",
        "алу" => "get",
        "рұқсатберу" => "allow",
        "дүрбелең" | "ойбай" => "panic",
        "мод" => "mod",
        "өзг" => "mut",
        "жаңа" => "new",
        "қайда" => "where",
        "үшін" => "for",
        "алу_немесе_енгізу" => "get_or_insert_with",
        "негізгі" => "main",
        "қоғы" => "pub",
        "сол" => None?,
        "қайтару" => "return",
        "асыру" => "impl",
        "сіл" => "ref",
        "сәйкестік" => "match",
        "егер" => "if",
        "басқа" => "else",
        "өзім" => "self",
        "беру" => "let",
        "статик" => "static",
        "құрыл" => "struct",
        "үміттену" => "expect",
        "ал" => "while",
        "пайдалану" => "use",
        "ішіне" => "into",
        "шын" => "true",
        "санақ" => "enum",
        "Топ" => "Group",
        "Идентиф" => "Ident",
        "ТокенАғыны" => "TokenStream",
        "ТокенАғашы" => "TokenTree",
        "жіпке" => "to_string",
        "жол_ретінде" => "as_str",
        "аралығы" => "span",
        "Век" => "Vec",
        "ағын" => "stream",
        "түрткі" => "push",
        "кеңейту" => "extend",
        "бөлгіш" => "delimiter",
        "Белг" => "Punct",
        "СөзбеСөз" => "Literal",
        "проц_макросы" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn тот(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
