/*
 * @Date: 2021-08-09 11:24:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-10 19:49:28
 */

#include <cassert>
#include <climits>
#include <queue>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int nthSuperUglyNumber_min_heap(int n, vector<int>& primes) {
    unordered_set<long> seen;
    priority_queue<long, vector<long>, greater<long>> heap;
    seen.insert(1);
    heap.push(1);
    int ugly = 0;
    for (int i = 0; i < n; i++) {
      long curr = heap.top();
      heap.pop();
      ugly = (int)curr;
      for (int prime : primes) {
        long next = curr * prime;
        if (seen.insert(next).second) {
          heap.push(next);
        }
      }
    }
    return ugly;
  }

  int nthSuperUglyNumber_dp(int n, vector<int>& primes) {
    vector<int> dp(n + 1);
    dp[1] = 1;
    int m = primes.size();
    vector<int> pointers(m, 1);
    for (int i = 2; i <= n; i++) {
      vector<int> nums(m);
      int minNum = INT_MAX;
      for (int j = 0; j < m; j++) {
        nums[j] = dp[pointers[j]] * primes[j];
        minNum = min(minNum, nums[j]);
      }
      dp[i] = minNum;
      for (int j = 0; j < m; j++) {
        if (minNum == nums[j]) {
          pointers[j]++;
        }
      }
    }
    return dp[n];
  }
};

int main() {
  {
    int n = 12;
    vector<int> primes{2, 7, 13, 19};
    int ans = 32;
    assert(Solution{}.nthSuperUglyNumber_min_heap(n, primes) == ans);
    assert(Solution{}.nthSuperUglyNumber_dp(n, primes) == ans);
  }

  {
    int n = 1;
    vector<int> primes{2, 3, 5};
    int ans = 1;
    assert(Solution{}.nthSuperUglyNumber_min_heap(n, primes) == ans);
    assert(Solution{}.nthSuperUglyNumber_dp(n, primes) == ans);
  }

  return 0;
}