
struct Solution;

// close but I'm stuck


fn min(a: i32, b: i32) -> i32 {
    if a > b {return b}
    return a
}
fn max(a: i32, b: i32) -> i32 {
    if a < b {return b}
    return a
}

fn find_median_single_arr(vec: Vec<i32>) -> f64 {
    let mid = vec.len()/2;
    println!("{}", mid);
    if vec.len()%2 == 0 {
        return (vec[mid-1] + vec[mid]) as f64 / 2.0;
    }
    return vec[mid] as f64
}

fn median_three(a: i32, b: i32, c: i32) -> i32 {
    let sum = a + b + c;
    let min_val = min(a, min(b , c));
    let max_val = max(a, max(b, c));
    return sum - min_val - max_val;
}

fn median_four(a: i32, b: i32, c: i32, d: i32) -> f64 {
    let sum = a + b + c + d;
    let min_val = min(a, min(b, min(c,d)));
    let max_val = max(a, max(b, max(c,d)));
    return (sum - max_val - min_val) as f64 / 2.0;
}

fn find_median_recursive(larger_vec: Vec<i32>, smaller_vec: Vec<i32>) -> f64 {
    println!("{:?}", larger_vec);
    println!("{:?}", smaller_vec);
    if smaller_vec.len() == 0 {
        return  find_median_single_arr(larger_vec)    
    }
    let larger_median = larger_vec.len()/2;

    if smaller_vec.len() == 1 {
        if larger_vec.len() == 1 {
            return find_median_single_arr(vec![smaller_vec[0], larger_vec[0]]);
        }
        if larger_vec.len()%2 == 1 {
            let comp_val = median_three(smaller_vec[0], larger_vec[larger_median-1], larger_vec[larger_median+1]);
            return find_median_single_arr(vec![larger_vec[larger_median], comp_val])
        }

            return median_three(larger_vec[larger_median], larger_vec[larger_median-1], smaller_vec[0]) as f64
        
    }
    else if smaller_vec.len() == 2 {
        if larger_vec.len() == 2 {
            return median_four(smaller_vec[0], smaller_vec[1], larger_vec[0], larger_vec[1])
        }
        if larger_vec.len() % 2 == 1 {
            return median_three(larger_vec[larger_median], max(smaller_vec[0], larger_vec[larger_median-1]), min(smaller_vec[1], larger_vec[larger_median + 1])) as f64
        }
        return median_four(larger_vec[larger_median], larger_vec[larger_median-1], max(smaller_vec[0], larger_vec[larger_median -2]), min(smaller_vec[1], larger_vec[larger_median + 1]))
    }

    let idx_sm = (smaller_vec.len() - 1 ) / 2;
    let idx_lg = (larger_vec.len() - 1) / 2;

    println!("idxsm {}, idxlg {}", idx_sm, idx_lg);

    if smaller_vec[idx_sm] <= larger_vec[idx_lg] {
        let (_leftsm, rightsm) = smaller_vec.split_at(idx_sm);
        let (leftlg, _rightlg) = larger_vec.split_at(idx_lg+1); 
        return find_median_recursive( leftlg.to_vec(), rightsm.to_vec())
    }
    let (leftsm, _rightsm) = smaller_vec.split_at(idx_sm+1);
    let (_leftlg, rightlg) = larger_vec.split_at(idx_lg); 
    return find_median_recursive(rightlg.to_vec(), leftsm.to_vec());
}



impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
      if nums1.len() > nums2.len() {
        return find_median_recursive(nums1, nums2);
      }
     return find_median_recursive(nums2, nums1)
    }   
}


fn main () {
   // println!("sol 1: 1?{}", Solution::find_median_sorted_arrays(vec![1], vec![2]));
    println!("sol 2: {}", Solution::find_median_sorted_arrays(vec![2,2,4,4], vec![2,2,4,4]));

}