//! Минимальный разбор строки CSV: запятые и поля в двойных кавычках.

/// Делит строку CSV на поля. Внутри кавычек запятая не разделяет поля,
/// а `""` означает одну кавычку.
pub fn fields(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.trim_end_matches('\r').chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => result.push(std::mem::take(&mut field)),
            _ => field.push(ch),
        }
    }
    result.push(field);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_by_comma() {
        assert_eq!(fields("a,b,c"), ["a", "b", "c"]);
    }

    #[test]
    fn keeps_empty_fields() {
        assert_eq!(fields("/d,dir,"), ["/d", "dir", ""]);
    }

    #[test]
    fn quoted_comma_stays_in_field() {
        assert_eq!(fields("\"a,b\",c"), ["a,b", "c"]);
    }

    #[test]
    fn doubled_quote_is_one_quote() {
        assert_eq!(fields("\"a\"\"b\",c"), ["a\"b", "c"]);
    }
}
