#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long maximumValueSum(vector<int> &nums, int k, vector<vector<int>> &edges) {
    long long f0 = 0, f1 = LLONG_MIN;
    for (int x : nums) {
      long long t = max(f1 + x, f0 + (x ^ k));
      f0 = max(f0 + x, f1 + (x ^ k));
      f1 = t;
    }
    return f0;
  }
};

int main() {
  vector<tuple<vector<int>, int, vector<vector<int>>, long long>> tests{
      {{1, 2, 1}, 3, {{0, 1}, {0, 2}}, 6},
      {{2, 3}, 7, {{0, 1}}, 9},
      {{7, 7, 7, 7, 7, 7}, 3, {{0, 1}, {0, 2}, {0, 3}, {0, 4}, {0, 5}}, 42},
  };

  for (auto &[nums,k, edges, ans] : tests) {
    assert(Solution().maximumValueSum(nums, k, edges) == ans);
  }
}