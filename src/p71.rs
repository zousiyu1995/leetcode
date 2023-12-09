#[test]
fn test() {
    // let path1: String = "/home/".to_string();
    // assert_eq!(simplify_path(path1), "/home".to_string());

    let path2: String = "/../".to_string();
    assert_eq!(simplify_path(path2), "/".to_string());

    let path3: String = "/home//foo/".to_string();
    assert_eq!(simplify_path(path3), "/home/foo".to_string());

    let path4: String = "/a/./b/../../c/".to_string();
    assert_eq!(simplify_path(path4), "/c".to_string());
}

pub fn simplify_path(path: String) -> String {
    let path: Vec<&str> = path.split("/").collect();
    let mut stack: Vec<&str> = vec![];

    // 遍历 path
    for s in path {
        match s {
            // 遇到当前目录`.`或者`/`，啥也不用干
            "." | "" => {}
            // 遇到跳转到上一个目录`..`，弹出栈顶元素，模拟跳转到上一个目录
            ".." => {
                stack.pop();
            }
            // 遇到目录名，压入栈中
            _ => stack.push(s),
        }
    }

    "/".to_string() + &stack.join("/")
}
