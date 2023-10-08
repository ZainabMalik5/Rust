///There is a robot on an m x n grid. 
///The robot is initially located at the top-left corner (i.e., grid[0][0]). 
///The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]).
/// The robot can only move either down or right at any point in time.
///Given the two integers m and n, return the number of possible unique paths 
///that the robot can take to reach the bottom-right corner.
fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![vec![0; n]; m];

    for i in 0..m {
        dp[i][0] = 1;
    }

    for i in 0..n {
        dp[0][i] = 1;
    }

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[m - 1][n - 1]
}

fn main() {
    let m = 3; // Replace with your values for m and n
    let n = 7;
    let result = unique_paths(m, n);
    println!("Number of unique paths: {}", result);
}
