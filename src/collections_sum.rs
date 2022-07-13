pub fn sum(vec: &[u32]) -> Option<u32>{
    let mut sum = 0;
    for item in vec {
        sum = sum + item;
    }
    if sum > 1000 {
        None
    } else {
        Some(sum)
    }
}