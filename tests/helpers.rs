
pub fn split_string<'a>(arg: &'a str) -> Vec<&'a str> {
    if arg.len() == 0 {
        return Vec::new();
    }
    let mut qstack: Vec<char> = Vec::new();
    let mut args: Vec<&str> = Vec::new();
    let mut start: usize = 0;
    let mut end = arg.len()-1;
    let mut iter = arg.chars().enumerate();
    let mut last: char = '\0';
    while let Some((i, c)) = iter.next() {
        if i == end {
            let trimmed = &arg[start..i+1].trim();
            if trimmed != &"" {
                args.push(trimmed);
            }
        } else if (c == '"' || c == '\'') && (i == 0 || (i != 0 && last != '\\')) {
            if qstack.len() != 0 {
                if qstack[qstack.len()-1] == c {
                    // remove the char from the stack
                    qstack.pop();
                }
            } else if i == 0 || (i != 0 && &arg[i-1..i] == " ") {
                // add a new quote to the stack
                qstack.push(c);
            }
        // } else if qstack.len() == 0 && c == ' ' && qstack.len() == 0 {
        } else if qstack.len() == 0 && c == ' ' {
            // do not add blank strings, accomplished via trim(), do not preserve spaces
            if i != start {
                let trimmed = &arg[start..i].trim();
                if trimmed != &"" {
                    args.push(trimmed);
                    start = i+1;
                }
            }
        }
        last = c;
    }
    args
}

pub fn split_string_preserve_spaces<'a>(arg: &'a str) -> Vec<&'a str> {
    if arg.len() == 0 {
        return Vec::new();
    }
    let mut qstack: Vec<char> = Vec::new();
    let mut args: Vec<&str> = Vec::new();
    let mut start: usize = 0;
    let mut end = arg.len()-1;
    let mut iter = arg.chars().enumerate();
    let mut last: char = '\0';
    while let Some((i, c)) = iter.next() {
        if i == end {
            let trimmed = &arg[start..i+1].trim();
            if trimmed != &"" {
                args.push(trimmed);
            }
        } else if (c == '"' || c == '\'') && (i == 0 || (i != 0 && last != '\\')) {
            if qstack.len() != 0 {
                if qstack[qstack.len()-1] == c {
                    // remove the char from the stack
                    qstack.pop();
                }
            } else if i == 0 || (i != 0 && &arg[i-1..i] == " ") {
                // add a new quote to the stack
                qstack.push(c);
            }
        // } else if qstack.len() == 0 && c == ' ' && qstack.len() == 0 {
        } else if qstack.len() == 0 && c == ' ' {
            // do not add blank strings (only spaces), but preserve the spaces
            if i != start && &arg[start..i].replace(" ", "") != "" {
                args.push(&arg[start..i]);
                start = i+1;
            }
        }
        last = c;
    }
    args
}