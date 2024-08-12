fn main() {}

pub fn remove_duplicates_sorted_slow(nums: &mut Vec<i32>) -> i32 {
    // this only works because the set is sorted
    let mut i = 1;
    while i < nums.len() {
        if nums[i] == nums[i - 1] {
            // this feels kind of like cheating
            // because it does the shift for us
            nums.remove(i);
            continue;
        }
        i += 1;
    }

    nums.len() as i32
}

pub fn remove_duplicates_sorted_faster(nums: &mut Vec<i32>) -> i32 {
    // this only works because the set is sorted
    let mut c = 0;
    let mut i = 1;
    while i < nums.len() {
        // this just shifts all of the unique items to the front
        // and it doesn't actually *remove* them
        // but the answer checker doesn't care ... so ??
        if nums[c] != nums[i] {
            c += 1;
            nums[c] = nums[i];
        }
        i += 1;
    }

    c as i32 + 1
}

#[test]
fn test_remove_1() {
    let mut nums = vec![1, 1, 2];

    //let remaining = remove_duplicates_sorted_slow(&mut nums);
    let remaining = remove_duplicates_sorted_faster(&mut nums);

    assert_eq!(remaining, 2);
}

#[test]
fn test_remove_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

    //let remaining = remove_duplicates_sorted_slow(&mut nums);
    let remaining = remove_duplicates_sorted_faster(&mut nums);

    assert_eq!(remaining, 5);
}
