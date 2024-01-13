/**
 * Assume you are an awesome parent, and want to give your children some cookies.
 * But you should give each child at most one cookie.
 * 
 * Each child [i] as a 'greed factor' g[i], which is the minimum size of a cookie
 * that the child will be content with, and each cookie [j] has size s[j].
 * 
 * If the size of cookie [j] is greater than or equal to the child's greed factor
 * g[i], we can assign the cookie j to the child i, and the child i will be content.
 * 
 * Maximize the number of your content children and output the maximum number.
 */
fn main() {
    let g1 = vec![1, 2, 3];
    let s1 = vec![1, 1];
    let result1 = Solution::find_content_children(g1, s1);

    let g2 = vec![1, 2];
    let s2 = vec![1, 2, 3];
    let result2 = Solution::find_content_children(g2, s2);

    println!("result = {}", result1);
    println!("result = {}", result2);
}

struct Solution {}
impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        if s.is_empty() || g.is_empty() {
            return 0;
        }

        g.sort_unstable();
        s.sort_unstable();

        let mut content_children = 0;

        for child in 0..g.len() {
            // find the smallest cookie that can satisfy the child
            // because the arrays are sorted, we can just iterate through the array
            for cookie in 0..s.len() {
                if g[child] <= s[cookie] {
                    content_children += 1;
                    s.remove(cookie);
                    break;
                }
            }

        }

        content_children
    }
}
