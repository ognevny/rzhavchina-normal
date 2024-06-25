use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Ошбк" => "Err",
        "Норм" => "Ok",
        "Строка" => "String",
        "КартаХэшей" => "HashMap",
        "Обычный" => "Default",
        "Ошибка" => "Error",
        "Опция" => "Option",
        "Чтото" => "Some",
        "Ничего" => "None",
        "Итог" => "Result",
        "Я" => "Self",
        "печатьлн" => "println",
        "печать" => "печать",
        "сломать" => "break",
        "асинхр" => "async",
        "ждать" => "await",
        "цикл" => "loop",
        "двигать" => "move",
        "ящик" => "crate",
        "недосягаемый_код" => "unreachable_code",
        "как" => "as",
        "пост" => "const",
        "типаж" => "trait",
        "небезопасный" => "unsafe",
        "в" => "in",
        "из" => "from",
        "дин" => "dyn",
        "развернуть" => "unwrap",
        "обычный" => "default",
        "как_ссылка" => "as_ref",
        "вв" => "io",
        "расширить" => "extern",
        "ложь" => "false",
        "фн" => "fn",
        "супер" => "super",
        "вставить" => "insert",
        "взять" => "get",
        "разрешить" => "allow",
        "паника" => "panic",
        "мод" => "mod",
        "мут" => "mut",
        "новый" => "new",
        "где" => "where",
        "для" => "for",
        "главный" => "main",
        "общий" => "pub",
        "вообщеничего" => None?,
        "вернуть" => "return",
        "внедрить" => "impl",
        "ссылка" => "ref",
        "совместить" => "match",
        "если" => "if",
        "иначе" => "else",
        "я" => "self",
        "пусть" => "let",
        "статичный" => "static",
        "структура" => "struct",
        "ожидать" => "expect",
        "пока" => "while",
        "исп" => "use",
        "превратить" => "into",
        "истина" => "true",
        "переч" => "enum",
        "шиза" => "union",
        "булево" => "bool",
        "Век" => "Vec",
        "век" => "vec",
        "толкнуть" => "push",
        "в_строку" => "to_string",
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
