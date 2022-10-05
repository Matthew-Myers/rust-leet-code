// Yuck.  Gross if statements.  Not O(log(m+n)) but O(m+n) memory
// REVIST

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut idx1 = 0;
        let mut idx2 = 0;
        let len = nums1.len() + nums2.len();
        let even: usize = len%2;
        
        let mut i = 0;
        let mut prev_item: f64 = 0.0;
        let mut second_prev_item: f64 = 0.0;
        while i <= len/2 + even {
            println!("i: {}, idx1: {}, idx2: {}", i, idx1, idx2);
            second_prev_item = prev_item;
            if idx1 == nums1.len() {
                prev_item = nums2[idx2] as f64;
                idx2 = idx2 + 1;
            } else if idx2 == nums2.len() {
                prev_item = nums1[idx1] as f64;
                idx1 = idx1 + 1;
            }
            else if nums1[idx1] <= nums2[idx2] {
                prev_item = nums1[idx1] as f64;
                idx1 = idx1 + 1;
            } else {
                prev_item = nums2[idx2] as f64;
                idx2 = idx2 + 1;
            }
            i = i+1;
        }
        if  even == 0 {
            return (prev_item + second_prev_item) / 2 as f64
        } else {
            return second_prev_item as f64
        }

    }   
}
