use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Ошбк" => "Err",
        "Норм" => "Ok",
        "Строчка" => "String",
        "КартаИзХэшей" => "HashMap",
        "Обычный" => "Default",
        "Нефартануло" => "Error",
        "Подстройка" => "Option",
        "Чето" => "Some",
        "Пусто" => "None",
        "Итог" => "Result",
        "Я" => "Self",
        "накалякать" => "println",
        "сломать" => "break",
        "ожидаемый" => "async",
        "ждать" => "await",
        "залупить" => "loop",
        "запивом" => "move",
        "закладка" => "crate",
        "где_эта_строчка" => "unreachable_code",
        "типо" => "as",
        "основа" => "const",
        "черта" => "trait",
        "вичположительный" => "unsafe",
        "глянуть" => "in",
        "украситьиз" => "from",
        "биполярка" => "dyn",
        "спиздить" => "unwrap",
        "каквсегда" => "default",
        "кинутьстрелку" => "as_ref",
        "иа" => "io",
        "счужогорайена" => "extern",
        "неправда" => "false",
        "пацикисделают" => "fn",
        "батя" => "super",
        "вставить" => "insert",
        "взять" => "get",
        "разрешаю" => "allow",
        "НАМКОНЕЦ" | "ПАМАГИТИ" | "ААААААААААА" => "panic",
        "порожняк" => "mod",
        "мутант" => "mut",
        "слепить" => "new",
        "гдеоно" => "where",
        "втечении" => "for",
        "отсюда" => "main",
        "район" => "pub",
        "вообщеничего" => None?,
        "отвечаю" => "return",
        "внутренность" => "impl",
        "кореш" => "ref",
        "футбол" => "match",
        "допустим" => "if",
        "забазаротвечай" => "else",
        "я" => "self",
        "пацик" => "let",
        "попонятиям" => "static",
        "типок" => "struct",
        "эээстой" => "expect",
        "поговорим" => "while",
        "поматросить" => "use",
        "вжопу" => "into",
        "четко" => "true",
        "клички" => "enum",
        "шиза" => "union",
        "попонятиямли" => "bool",
        "Банда" => "Vec",
        "банда" => "vec",
        "запихнуть" => "push",
        "застрочить" => "to_string",
        "получить_или_вставить" => "get_or_insert_with",

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
pub fn rzhavchina(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
