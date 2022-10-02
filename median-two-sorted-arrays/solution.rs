// Yuck.  Gross if statements.  Not O(log(m+n)) but O(m+n) memory
// REVIST

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut idx1 = 0;
        let mut idx2 = 0;
        let len = nums1.len() + nums2.len();
        let even: usize = if let 0=len%2 {
            0
        } else {
            1
        };
        
        let mut i = 0;
        let mut previousItem: f64 = 0.0;
        let mut secondPreviousItem: f64 = 0.0;
        while i <= len/2 + even {
            secondPreviousItem = previousItem;
            if idx1 == nums1.len() {
                previousItem = nums2[idx2] as f64;
                idx2 = idx2 + 1;
            } else if idx2 == nums2.len() {
                previousItem = nums1[idx1] as f64;
                idx1 = idx1 + 1;
            }
            else if nums1[idx1] <= nums2[idx2] {
                previousItem = nums1[idx1] as f64;
                idx1 = idx1 + 1;
            } else {
                previousItem = nums2[idx2] as f64;
                idx2 = idx2 + 1;
            }
            i = i+1;
        }
        if  even == 0 {
            return ((previousItem + secondPreviousItem) / 2 as f64)
        } else {
            return secondPreviousItem as f64
        }

    }   
}