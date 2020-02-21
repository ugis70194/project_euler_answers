pub fn solve() -> i64 {
    let mut ans: i64 = 0;
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            ans += x;
        }
    }
    ans
}