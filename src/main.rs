mod tests;

fn main() {
}

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;

    if nums.is_empty() { return -1; }

    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return mid,
        }
    }

    -1
}

pub fn shuffle(nums: &mut Vec<i32>) {
    use rand::Rng;

    if nums.len() < 2 { return; }

    let mut rng = rand::thread_rng();

    for i in 1..nums.len() {
        let r: usize = rng.gen_range(0..=i);
        if r != i {
            nums.swap(i, r);
        }
    }
}

pub fn get_prefix_sum(nums: &[i32]) -> Vec<i32> {
    if nums.is_empty() { return Vec::new(); }

    let mut res = Vec::with_capacity(nums.len());
    res.push(nums[0]);

    for i in 1..nums.len() {
        res.push(res[i - 1] + nums[i]);
    }

    res
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 { return -1; }

    let prefix_sum = get_prefix_sum(&nums);
    let total_sum = prefix_sum.last().unwrap();

    for i in 1..prefix_sum.len() - 1 {
        if prefix_sum[i - 1] == total_sum - prefix_sum[i] {
            return i as i32;
        }
    }

    -1
}

pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
    if nums.is_empty() || k <= 0 { return; }

    k = k % nums.len() as i32;
    if k == 0 { return; }

    nums.reverse();
    nums[..k as usize].reverse();
    nums[k as usize..].reverse();
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    use std::cmp::max;

    let mut max_sum = nums[0];
    let mut curr_sum = nums[0];
    for n in nums.into_iter().skip(1) {
        curr_sum = max(curr_sum + n, n);
        max_sum = max(max_sum, curr_sum);
    }
    max_sum
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::{max, min};

    let mut max_profit = 0;
    let mut buy = prices[0];

    for p in prices.into_iter().skip(1) {
        max_profit = max(max_profit, p - buy);
        buy = min(buy, p);
    }

    max_profit
}