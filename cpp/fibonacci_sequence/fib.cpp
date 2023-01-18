/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 09:46:27
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-19 09:46:51
 */

#include <cstdio>
#include <string>
#include <vector>

int just_recursive(int N) {
  if (N == 1 || N == 2) return 1;
  return just_recursive(N - 1) + just_recursive(N - 2);
}

int dp_fib(int N) {
  std::vector<int> dp(N + 1, 0);

  dp[1] = dp[2] = 1;
  for (int i = 3; i <= N; ++i) {
    dp[i] = dp[i - 1] + dp[i - 2];
  }
  return dp[N];
}

int fib(int n) {
  if (n == 2 || n == 1) return 1;
  int prev = 1, curr = 1, sum = 0;
  for (int i = 3; i <= n; ++i) {
    sum = prev + curr;
    prev = curr;
    curr = sum;
  }
  return curr;
}

int main() {
  printf("The result = %d\n", just_recursive(45));
  printf("The result = %d\n", dp_fib(45));
  printf("The result = %d\n", fib(45));
  return 0;
}
