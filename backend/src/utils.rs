pub trait InputUtils {
    fn split_sections(&self) -> Vec<String>;
}

impl InputUtils for str {
    fn split_sections(&self) -> Vec<String> {
        return self
            .replace("  ", "")
            .split("\n\n")
            .map(|x| String::from(x.trim()))
            .filter(|x| !x.is_empty())
            .collect();
    }
}
