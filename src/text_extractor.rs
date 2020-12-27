use regex::Regex;

pub fn extract_string_from_row(row: &str, regex: Regex) -> Result<String, &str> {
    let texts = regex.captures(row);
    let text = match texts {
        None => return Err("failed"),
        Some(value) => String::from(&value[0])
    };

    let split: Vec<&str> = text.split("=").collect();
    return match split[1].parse::<String>() {
        Err(_) => Err("failed"),
        Ok(value) => Ok(value)
    };
}

pub fn extract_duration_from_row(row: &str, regex: Regex) -> Result<f32, &str> {
    let captured = match regex.captures(row) {
        None => return Err("failed"),
        Some(value) => value
    };

    let split: Vec<&str> = captured[0].split("=").collect();
    return match split[1].parse::<f32>() {
        Err(_) => Err("failed"),
        Ok(value) => Ok(value)
    };
}

pub fn extract_page_from_row(row: &str) -> Result<String, &str> {
    let path_regex = Regex::new(r"path=(\S+)").unwrap();
    let method_regex = Regex::new(r"method=(\S+)").unwrap();

    let path = extract_string_from_row(row, path_regex)?;
    let method = extract_string_from_row(row, method_regex)?;

    return Ok(format!("{} {}", method, path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_string_from_row_should_return_controller_name_when_a_row_has_controller_name() {
        let row = "[xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx] method=GET path=/foo format=html controller=FooController action=index status=200";
        let regex = regex::Regex::new(r"controller=(\S+)").unwrap();

        let value = extract_string_from_row(row, regex);
        assert_eq!(Ok(String::from("FooController")), value);
    }

    #[test]
    fn extract_string_from_row_should_return_err_when_a_row_does_not_have_controller_name() {
        let row = "path=/foo aaaaa bbbbb ccccc";
        let regex = regex::Regex::new(r"controller=(\S+)").unwrap();

        let value = extract_string_from_row(row, regex);
        assert_eq!(Err("failed"), value);
    }

    #[test]
    fn extract_duration_from_row_should_return_duration_when_a_row_has_duration() {
        let row = "method=GET path=/foo format=html controller=FooController action=index status=200 duration=1915.45 view=1841.20 db=7.93";
        let value = extract_duration_from_row(row, Regex::new(r"duration=(\S+)").unwrap());
        assert_eq!(Ok(1915.45), value);
    }

    #[test]
    fn extract_duration_from_row_should_return_err_when_a_row_does_not_have_controller_name() {
        let row = "path=/foo aaaaa bbbbb ccccc";
        let value = extract_duration_from_row(row, Regex::new(r"duration=(\S+)").unwrap());
        assert_eq!(Err("failed"), value);
    }

    #[test]
    fn extract_duration_from_row_should_return_view_when_view_regex_is_given() {
        let row = "method=GET path=/foo format=html controller=FooController action=index status=200 duration=1915.45 view=1841.20 db=7.93";
        let value = extract_duration_from_row(row, Regex::new(r"view=(\S+)").unwrap());
        assert_eq!(Ok(1841.20), value);
    }

    #[test]
    fn extract_page_from_row_should_return_method_and_path() {
        let row = "[xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx] method=GET path=/foo format=html controller=FooController action=index status=200";
        let value = extract_page_from_row(row);
        assert_eq!(Ok(String::from("GET /foo")), value);
    }

    #[test]
    fn extract_page_from_row_should_return_empty_string_when_a_row_does_not_have_enough_information() {
        let row = "aaaaa bbbbb ccccc";
        let value = extract_page_from_row(row);
        assert_eq!(Err("failed"), value);
    }
}
