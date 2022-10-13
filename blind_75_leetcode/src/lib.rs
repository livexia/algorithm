mod solution;

#[macro_export]
macro_rules! leetcode_vec {
    ($($x: expr),* $(,)?) => {
        {
            let mut temp_vce = Vec::new();
            $(
                temp_vce.push($x.to_vec());
            )*
            temp_vce
        }
    }
}
