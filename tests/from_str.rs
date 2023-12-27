#[cfg(test)]
mod tests {
    use black_box::*;

    #[derive(Clone, Debug, FromStr)]
    struct MyStruct {
        pub data: i64,
    }

    impl From<&str> for MyStruct {
        fn from(_item: &str) -> Self {
            return MyStruct {
                data: _item.parse::<i64>().unwrap(),
            };
        }
    }

    #[test]
    fn test_from_string_derived_from_str() {
        let s: String = "1".to_string();
        let a = MyStruct::from(s.clone());

        assert_eq!(a.data, 1);
        assert_eq!(a.data.to_string(), s);
        assert_eq!(a.data.to_string().as_str(), s.as_str());
    }
}
