use proptest::prelude::*;

use logos::Logos;

use compiler::C1Token;

proptest! {
    #[test]
    fn protest_all_tokens(
        t in 0u32..34,
        int_val in 0i64..i64::MAX,
        float_val in r"([0-9]+\.[0-9]+)|(\.[0-9]+([eE]([-+])?[0-9]+)?)|([0-9]+[eE]([-+])?[0-9]+)",
        bool_val in "true|false",
        str_val in "\"[^\n\"]*\"",
        id_val in "[a-zA-Z]+[0-9a-zA-Z]*",
    ) {
        let mut input_string = vec![];
        let mut output_tokens:Vec<String> = vec![];

        let val = t;
        let int_val_str = int_val.to_string();
        let float_val_str = format!("{:.0}", float_val).to_string();
        let bool_val_str = bool_val.to_string();
        let str_val_str = str_val.to_string();
        let id_val_str = id_val.to_string();

        match val {
            0 => {
                input_string.push("bool");
                output_tokens.push("KwBoolean: \"bool\"".to_string());
            },
            1 => {
                input_string.push("do");
                output_tokens.push("KwDo: \"do\"".to_string());
            },
            2 => {
                input_string.push("else");
                output_tokens.push("KwElse: \"else\"".to_string());
            },
            3 => {
                input_string.push("for");
                output_tokens.push("KwFor: \"for\"".to_string());
            },
            4 => {
                input_string.push("if");
                output_tokens.push("KwIf: \"if\"".to_string());
            },
            5 => {
                input_string.push("int");
                output_tokens.push("KwInt: \"int\"".to_string());
            },
            6 => {
                input_string.push("printf");
                output_tokens.push("KwPrintf: \"printf\"".to_string());
            },
            7 => {
                input_string.push("return");
                output_tokens.push("KwReturn: \"return\"".to_string());
            },
            8 => {
                input_string.push("void");
                output_tokens.push("KwVoid: \"void\"".to_string());
            },
            9 => {
                input_string.push("String");
                output_tokens.push("KwString: \"String\"".to_string());
            },
            10 => {
                input_string.push("while");
                output_tokens.push("KwWhile: \"while\"".to_string());
            },
            11 => {
                input_string.push("+");
                output_tokens.push("Plus: \"+\"".to_string());
            },
            12 => {
                input_string.push("-");
                output_tokens.push("Minus: \"-\"".to_string());
            },
            13 => {
                input_string.push("*");
                output_tokens.push("Asterisk: \"*\"".to_string());
            },
            14 => {
                input_string.push("/");
                output_tokens.push("Slash: \"/\"".to_string());
            },
            15 => {
                input_string.push("=");
                output_tokens.push("Assign: \"=\"".to_string());
            },
            16 => {
                input_string.push("==");
                output_tokens.push("Eq: \"==\"".to_string());
            },
            17 => {
                input_string.push("!=");
                output_tokens.push("Neq: \"!=\"".to_string());
            },
            18 => {
                input_string.push("<");
                output_tokens.push("LSS: \"<\"".to_string());
            },
            19 => {
                input_string.push(">");
                output_tokens.push("GRT: \">\"".to_string());
            },
            20 => {
                input_string.push("<=");
                output_tokens.push("LEQ: \"<=\"".to_string());
            },
            21 => {
                input_string.push(">=");
                output_tokens.push("GEQ: \">=\"".to_string());
            },
            22 => {
                input_string.push("&&");
                output_tokens.push("And: \"&&\"".to_string());
            },
            23 => {
                input_string.push("||");
                output_tokens.push("Or: \"||\"".to_string());
            },
            24 => {
                input_string.push(",");
                output_tokens.push("Comma: \",\"".to_string());
            },
            25 => {
                input_string.push(";");
                output_tokens.push("Semicolon: \";\"".to_string());
            },
            26 => {
                input_string.push("(");
                output_tokens.push("LParen: \"(\"".to_string());
            },
            27 => {
                input_string.push(")");
                output_tokens.push("RParen: \")\"".to_string());
            },
            28 => {
                input_string.push("{");
                output_tokens.push("LBrace: \"{\"".to_string());
            },
            29 => {
                input_string.push("}");
                output_tokens.push("RBrace: \"}\"".to_string());
            },
            30 => {
                input_string.push(&int_val_str);
                output_tokens.push(
                    format!("ConstInt({}): \"{}\"", int_val_str, int_val_str).to_string()
                );
            },
            31 => {
                input_string.push(&float_val_str);
                output_tokens.push(
                    format!("ConstFloat({}): \"{}\"", float_val_str, float_val_str).to_string()
                );
            },
            32 => {
                input_string.push(&bool_val_str);
                output_tokens.push(
                    format!("ConstBoolean({}): \"{}\"", bool_val_str, bool_val_str).to_string()
                );
            },
            33 => {
                input_string.push(&str_val_str);
                output_tokens.push(
                    format!("ConstString({:?}): {:?}", str_val_str, str_val_str).to_string()
                );
            },
            34 => {
                input_string.push(&id_val_str);
                output_tokens.push(
                    format!("Id({}): \"{}\"", id_val_str, id_val_str).to_string()
                );
            },
            _ => {
                println!("{:?}", val);
            },
        }

        let test_input = input_string.join(" ");

        let mut lexer = C1Token::lexer(test_input.as_str());

        let mut tokens = Vec::new();
        while let Some(val) = lexer.next() {
            let parse_result = format!("{:?}: {:?}", val, lexer.slice());
            tokens.push(parse_result);
        }

        for (parsed, expected) in tokens.iter().zip(output_tokens) {
        //    println!("{} {}", parsed, expected);
            assert_eq!(parsed.to_string(), expected);
        }
    }
}