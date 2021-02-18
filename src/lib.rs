pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    if n == 1 {
        return result
    }
    let mut num = n; 
    let mut count: u64 = 2;
    while (count as f32) <= (num as f32).sqrt() {
        dbg!(&count);
        if (num % count) == 0{
            dbg!(&count);
            dbg!(&num);
            result.push(count);
            num = num / count;
            count = 2;
        } else {
            count = count + 1;
        }
    }
    result.push(num);
    result
}
