mod tests;
mod list_node;
mod linked_list;

use list_node::ListNode;

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

// 33. Search in Rotated Sorted Array.
// https://leetcode.com/problems/search-in-rotated-sorted-array/
// Solution with O(logN) time and O(1) space.
pub fn binary_search_rotated(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() { return -1; }
    if nums.len() == 1 {
        return if nums[0] == target { 0 } else { -1 };
    }

    use std::cmp::Ordering;

    fn search_smallest(nums: &[i32]) -> usize {
        let mut low = 0;
        let mut high = nums.len() - 1;
        if nums[low] < nums[high] { return 0; }

        let mut mid = 0;
        while low <= high {
            mid = low + (high - low) / 2;
            if nums[mid] > nums[mid + 1] {
                break;
            } else {
                if nums[mid] < nums[low] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }
        mid + 1
    }

    let smallest = search_smallest(&nums);
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    if target == nums[smallest] {
        return smallest as i32;
    }

    if smallest != 0 {
        if target >= nums[0] {
            high = smallest as i32 - 1;
        } else {
            low = smallest as i32 + 1
        }
    }

    while low <= high {
        let mid = low + (high - low) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Equal => return mid,
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
        }
    }

    -1
}

// Solution with O(logN) time and O(1) space.
pub fn binary_search_rotated_v2(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if target == nums[mid as usize] { return mid; }

        if nums[mid as usize] >= nums[low as usize] {
            if target >= nums[low as usize] && target < nums[mid as usize] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if target > nums[mid as usize] && target <= nums[high as usize] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }

    -1
}

// 977. Squares of a Sorted Array.
// https://leetcode.com/problems/squares-of-a-sorted-array/
// Solution with O(N) time and O(N) space.
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() { return nums; }

    let mut res = vec![0; nums.len()];
    let mut low = 0;
    let mut high = nums.len() - 1;

    let mut i = high as i32;
    while low <= high {
        if nums[low].abs() >= nums[high].abs() {
            res[i as usize] = nums[low].pow(2);
            low += 1;
        } else {
            res[i as usize] = nums[high].pow(2);
            high -= 1;
        }
        i -= 1;
    }

    res
}

// 34. Find First and Last Position of Element in Sorted Array.
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// Solution with O(logN) time and O(1) space.
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;

    if nums.is_empty() { return vec![-1, -1]; }

    let mut low: i32 = 0;
    let mut high: i32 = nums.len() as i32 - 1;

    let mut start = -1;
    while low <= high {
        let mid = low + (high - low) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Equal => {
                start = mid;
                high = mid - 1;
            },
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
        }
    }

    if start == -1 { return vec![-1, -1]; }
    let mut end = start;

    low = start + 1;
    high = nums.len() as i32 - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        match nums[mid as usize].cmp(&target) {
            Ordering::Equal => {
                end = mid;
                low = mid + 1;
            },
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
        }
    }

    vec![start, end]
}

// 206. Reverse Linked List.
// https://leetcode.com/problems/reverse-linked-list/
// Solution with O(N) time and O(1) space.
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn solve(mut prev_node: Option<Box<ListNode>>, mut curr_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut curr_node_inner) = curr_node {
            let next_node = curr_node_inner.next.take();
            curr_node_inner.next = prev_node.take();
            prev_node = Some(curr_node_inner);
            curr_node = next_node;
            solve(prev_node, curr_node)
        } else {
            prev_node
        }
    }

    solve(None, head)
}