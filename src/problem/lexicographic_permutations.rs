use super::Problem;

fn swap(arr: &mut [u8; 10], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

// Algorithm based on: https://www.baeldung.com/cs/array-generate-all-permutations#bd-permutations-in-lexicographic-order
fn get_next_permutation(arr: &mut [u8; 10]) -> bool {
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }

    if i <= 0 {
        return false;
    }

    let mut j = arr.len();
    while j > i && arr[j - 1] <= arr[i - 1] {
        j -= 1;
    }

    swap(arr, i -1, j - 1);
    
    i = i + 1;
    j = arr.len();
    while i < j {
        swap(arr, i -1, j - 1);
        i += 1;
        j -= 1;
    }
 
    return true;
}

pub struct LexicographicPermutationsProblem {}

impl Problem for LexicographicPermutationsProblem {
    fn name(&self) -> String {
        String::from("Lexicographic Permutations")
    }

    fn number(&self) -> u16 {
        24    
    }
    
    fn solve(&self) -> String {
        let mut start_permutation: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut perm_count = 1;

        while perm_count < 1_000_000 {
            get_next_permutation(&mut start_permutation);
            perm_count += 1;
        }

        return format!("{:?}", start_permutation);
    }
}
