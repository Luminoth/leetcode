fn main() {}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // we COULD do this with retain() but that feels like cheating

    let mut i = 0;
    while i < nums.len() {
        if nums[i] == val {
            nums.swap_remove(i);
            continue;
        }
        i += 1;
    }

    nums.len() as i32
}

#[test]
fn test_remove_1() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;

    let remaining = remove_element(&mut nums, val);

    assert_eq!(remaining, 2);
}

#[test]
fn test_remove_2() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;

    let remaining = remove_element(&mut nums, val);

    assert_eq!(remaining, 5);
}
