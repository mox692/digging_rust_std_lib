#[allow(unused_macro_rules, unused_macros)]
macro_rules! vec {
    // パターン: 空のリスト
    () => {
        Vec::new()
    };

    // パターン: コンマ区切りの要素
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = crate::vec::Vec::new();
            $(
                temp_vec.push($x);
            )+
            temp_vec
        }
    };
}

#[test]
fn s() {
    let _: crate::vec::Vec<i32> = vec![1, 2, 3];
    let _: crate::vec::Vec<&str> = vec!["a", "b", "c"];
}
