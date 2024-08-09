fn main() {}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // nothing to do in this case
    if n == 0 {
        return;
    }

    let m = m as usize;
    let n = n as usize;

    let mut i = 0_usize;
    let mut j = 0_usize;

    loop {
        // if we've run past nums1 then copy whatever is left from nums2
        if i >= m {
            // if we've also run past nums2 then we're done
            if j >= n {
                break;
            }

            nums1[i] = nums2[j];

            i += 1;
            j += 1;
            continue;
        }

        // if the item in nums1 is larger, swap it with the item in nums2
        if nums1[i] > nums2[j] {
            let v = nums1[i];
            nums1[i] = nums2[j];

            // but we need to keep nums2 sorted
            let mut x = j + 1;
            loop {
                if x >= n {
                    nums2[x - 1] = v;
                    break;
                }

                if nums2[x] < v {
                    nums2[x - 1] = nums2[x];
                    x += 1;
                    continue;
                }

                nums2[x - 1] = v;
                break;
            }

            continue;
        }

        // otherwise keep moving along nums1
        i += 1;
    }
}

#[test]
fn test_merge_1() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    merge(&mut nums1, 3, &mut nums2, 3);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn test_merge_2() {
    let mut nums1 = vec![1];
    let mut nums2 = vec![];

    merge(&mut nums1, 1, &mut nums2, 0);

    assert_eq!(nums1, vec![1]);
}

#[test]
fn test_merge_3() {
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];

    merge(&mut nums1, 0, &mut nums2, 1);

    assert_eq!(nums1, vec![1]);
}

#[test]
fn test_merge_4() {
    let mut nums1 = vec![7, 8, 9, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    merge(&mut nums1, 3, &mut nums2, 3);

    assert_eq!(nums1, vec![2, 5, 6, 7, 8, 9]);
}
