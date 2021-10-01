#![feature(exclusive_range_pattern)]

trait ColorExtend {
    fn is_color(&self) -> bool;
}

impl ColorExtend for &str {
    fn is_color(&self) -> bool {
        if self.len() == 4 || self.len() == 7 { 
            for elem in self.char_indices().next() {
                match elem {
                    (0, '#') => (),
                    (0, _) => return false,
                    (_, '0'..'9') => (),
                    (_, 'a'..'z') => (),
                    (_, 'A'..'Z') => (),
                    _ => return false,
                }
            }
            true
        } else {
            false
        }
    }
}

trait TryAdd {
    type Target;
    fn try_add(&self, other: Self::Target) -> Self;
}

impl TryAdd for Option<String> {
    type Target = Option<String>;

    fn try_add(&self, other: Self::Target) -> Self {
        match (self, &other) {
            (Some(left), Some(right)) => Some(format!("{}{}", left, right)),
            (None, Some(right)) => Some(right.clone()),
            (Some(left), None) => Some(left.clone()),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color() {
        let str = "#000";
        assert!(str.is_color());
        let str = "#F41CBA";
        assert!(str.is_color());
        let str = "F41CBA";
        assert!(!str.is_color());
        let str = "F41CBAA";
        assert!(!str.is_color());
        let str = "!asd;";
        assert!(!str.is_color());
    }

    #[test]
    fn test_try_add() {
        let province = Some(String::from("北京"));
        let city = Some(String::from("天安门"));
        assert_eq!(province.try_add(city), Some(String::from("北京天安门")));

        let province = Some(String::from("张三"));
        let city = None;
        assert_eq!(province.try_add(city), Some(String::from("张三")));

        let str: Option<String> = None;
        let ret = str.try_add(Some("电脑".to_string()));
        assert_eq!(ret, Some("电脑".to_string()))
    }
}