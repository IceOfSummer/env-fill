use std::collections::HashMap;

///
/// 替换字符串中的占位符.
///
/// # Examples
///
/// ```rust
/// let mut map = std::collections::HashMap::new();
/// map.insert(String::from("who"), String::from("world"));
/// let result = env_fill::replacer::replace_content(&String::from("hello {who}!"), &map);
///
/// assert_eq!(result, String::from("hello world!"));
/// ```
pub fn replace_content(content: &String, values: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut key = String::new();
    
    let mut chs = content.chars();
    while let Some(ch) = chs.next() {
        if ch == '{' {
            // search for end.
            let mut append_end = true;
            while let Some(kch) = chs.next() {
                if kch == '}' {
                    break;
                }
                key.push(kch);
                if kch == '\n' {
                    append_end = false;
                    break;
                }
            }
            match values.get(&key) {
                Some(val) => result.push_str(val),
                _ => {
                    result.push('{');
                    result.push_str(&key);
                    if append_end {
                        result.push('}')
                    }
                }
            }
            key = String::new();
            continue;
        }
        result.push(ch);
    }
    result
}