pub fn solve() -> u64 {
    let four_million : u64 = 4000000;
    let mut fibonacci : Vec<u64> = Vec::new();
    fibonacci.push(1);
    fibonacci.push(2);

    let mut ans : u64 = 2;
    let mut idx : usize = 1;
    while fibonacci.last() < Some(&four_million) {
        let nxt_term : u64 = fibonacci[idx-1] + fibonacci[idx];
        if nxt_term % 2 == 0 { ans += nxt_term; }
        fibonacci.push(nxt_term);
        idx += 1;
    }
    ans
}