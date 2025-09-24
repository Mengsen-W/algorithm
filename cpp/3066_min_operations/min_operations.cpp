#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int> &nums, int k) {
    int res = 0;
    priority_queue<long long, vector<long long>, greater<long long>> pq(nums.begin(), nums.end());
    while (pq.top() < k) {
      long long x = pq.top();
      pq.pop();
      long long y = pq.top();
      pq.pop();
      pq.push(x + x + y);
      res++;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{2, 11, 10, 1, 3}, 10, 2},
      {{1, 1, 2, 4, 9}, 20, 4},
  };
  

  for (auto &[nums, k, ans] : tests) {
    assert(Solution().minOperations(nums, k) == ans);
  }
}