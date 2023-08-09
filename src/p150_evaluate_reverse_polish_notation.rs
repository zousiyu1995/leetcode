#[test]
fn test() {
    let tokens1: Vec<String> = ["2", "1", "+", "3", "*"].map(|s| s.to_string()).to_vec();
    assert_eq!(eval_rpn(tokens1), 9);

    let tokens2: Vec<String> = ["4", "13", "5", "/", "+"].map(|s| s.to_string()).to_vec();
    assert_eq!(eval_rpn(tokens2), 6);

    let tokens3: Vec<String> = [
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .map(|s| s.to_string())
    .to_vec();
    assert_eq!(eval_rpn(tokens3), 22);
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    let operator: Vec<String> = ["+", "-", "*", "/"].map(|s| s.to_string()).to_vec();

    for s in tokens {
        // 遇到运算符，计算。结果重新压入栈中
        if operator.contains(&s) {
            let r: i32 = stack.pop().unwrap(); // 先出栈的是右操作数
            let l: i32 = stack.pop().unwrap(); // 后出栈的是左操作数
            match s.as_str() {
                "+" => stack.push(l + r),
                "-" => stack.push(l - r),
                "*" => stack.push(l * r),
                "/" => stack.push(l / r),
                _ => unreachable!(),
            }
        }
        // 遇到数字，压入栈中
        else {
            let num: i32 = s.parse::<i32>().unwrap();
            stack.push(num);
        }
    }

    stack.pop().unwrap()
}
