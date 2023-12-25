#[cfg(test)]
mod tests {
    use black_box::*;

    #[derive(Clone, Debug, PartialEqSigned, PartialEqUnsigned)]
    struct MyStruct {
        pub data: i64,
    }

    impl PartialEq<i64> for MyStruct {
        fn eq(&self, other: &i64) -> bool {
            self.data == *other
        }
    }

    impl PartialEq<u64> for MyStruct {
        fn eq(&self, other: &u64) -> bool {
            (self.data as u64) == *other
        }
    }

    #[test]
    fn test_partial_eq_from_partial_eq_64() {
        let a = MyStruct { data: -1 };

        assert_eq!(a, -1 as i64);
        assert_eq!(a, -1 as i32);
        assert_eq!(a, -1 as i16);
        assert_eq!(a, -1 as i8);

        assert_ne!(a, 1 as u64);
        assert_ne!(a, 1 as u32);
        assert_ne!(a, 1 as u16);
        assert_ne!(a, 1 as u8);
    }
}
