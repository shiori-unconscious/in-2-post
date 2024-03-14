/** 程序功能：将中缀表达式转换为后缀表达式的翻译器
  程序输入：有符号的整数、浮点数，可使用科学计数法，以及'+''-''*''/''('')'构成的中缀表达式
            例如: 在键盘上输入9e9-5e2+(2e3+3.14*114.321)*7.1/9*(9.1+5.55e-1)
  原始文法描述：
            expr --> expr + term
                    | expr - term
                    | term
            term --> term * factor
                    | term / factor
                    | factor
            factor --> (expr) | num

  消除左递归后的语法制导定义：
        产生式                      语义规则
        expr --> term  rest         expr.syn = term.syn || rest.syn

        rest --> + term rest1       rest.syn = term.syn || '+' || rest1.syn
                | - term rest1      rest.syn = term.syn || '-' || rest1.syn
                | 空                rest.syn = ""

        term --> factor other       term.syn = factor.syn || other.syn

        other --> * term other1     other.syn = term.syn || '*' || other1.syn
                | / term other1     other.syn = term.syn || '/' || other1.syn
                | 空                other.syn = ""

        factor --> (expr)           factor.syn = '(' || expr.syn || ')'
                | num               factor.syn = num.syn
   说明：综合属性的英文翻译为: synthesized attribute
         expr.syn表示expr的一个综合属性为syn，该属性存储expr的后缀表达式串
*/
use std::io;
use std::str::Chars;

fn expr(look_ahead: &mut Chars) -> String {
    format!("{}{}", term(look_ahead), rest(look_ahead))
}

fn term(look_ahead: &mut Chars) -> String {
    match look_ahead.next() {
        Some(digit @ '0'..='9') => String::from(digit),
        _ => panic!("invalid term found"),
    }
}

fn rest(look_ahead: &mut Chars) -> String {
    match look_ahead.next() {
        Some(opt @ ('+' | '-')) => {
            format!("{}{}{}", term(look_ahead), opt, rest(look_ahead))
        }
        Some(err) => panic!("unknown operator in source: {}", err),
        None => "".to_string(),
    }
}

fn main() {
    println!("please input source code");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read src from stdin");
    let src = buffer.trim(); // remove '\r\n' character

    let mut look_ahead = src.chars();
    let res = expr(&mut look_ahead);
    if let Some(err) = look_ahead.next() {
        panic!("invalid input source code: {}", err);
    }
    println!("finished parse, the result of post order is {}", res);
}
