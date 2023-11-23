#[cfg(test)]
mod tests {
    use black_box::*;
    use core::ops::*;

    #[derive(Clone, Debug, PartialEq, Add, Sub)]
    struct MyStruct<const N: usize> {
        pub data: [i32; N],
    }

    impl<const N: usize> AddAssign<MyStruct<N>> for MyStruct<N> {
        fn add_assign(&mut self, rhs: MyStruct<N>) {
            self.data[0] += rhs.data[0];
            self.data[1] += rhs.data[1];
        }
    }

    impl<const N: usize> SubAssign<MyStruct<N>> for MyStruct<N> {
        fn sub_assign(&mut self, rhs: MyStruct<N>) {
            self.data[0] -= rhs.data[0];
            self.data[1] -= rhs.data[1];
        }
    }

    #[test]
    fn test_op_from_op_assign() {
        let a = MyStruct::<2> { data: [1, 1] };
        let b = MyStruct::<2> { data: [1, 1] };
        let c = a + b;

        assert_eq!(c.data, [2, 2]);
    }
}
