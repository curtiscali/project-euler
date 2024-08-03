use crate::problem::Problem;

fn reverse(n: i32) -> i32 {
    let mut rev: i32 = 0;
    let mut i = n;

    while i > 0 {
        rev = rev * 10;
        rev = rev + (i % 10);
        i = i / 10;
    }
    

    return rev;
}

fn is_palindrome(n: i32) -> bool {
    return n == reverse(n);
}

pub struct LargestPalindromeProduct {
    pub limit: i32
}

impl Problem for LargestPalindromeProduct {
    fn solve(&self) -> String {
        let mut largest_palindrome: i32 = 0;
        let mut products: (i32, i32) = (0, 0);

        // We can start at half the limit, since the largest palindrome is likely to be
        // towards the top of the range rather than the bottom. This allows us to cut our problem set in half
        let mut i: i32 = self.limit / 2;
        while i < self.limit {
            let mut j: i32 = self.limit / 2;
            while j < self.limit {
                if i * j > largest_palindrome && is_palindrome(i * j) {
                    products.0 = i;
                    products.1 = j;

                    largest_palindrome = i * j;
                }

                j = j + 1
            }

            i = i + 1;
        }

        return format!("{} * {} = {}", products.0, products.1, largest_palindrome);
    }
}