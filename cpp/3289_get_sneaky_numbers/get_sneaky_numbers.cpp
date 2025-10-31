#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> getSneakyNumbers(vector<int>& nums) {
    int n = (int)nums.size() - 2;
    int sum = 0, squaredSum = 0;
    for (int x : nums) {
      sum += x;
      squaredSum += x * x;
    }
    int sum2 = sum - n * (n - 1) / 2;
    int squaredSum2 = squaredSum - n * (n - 1) * (2 * n - 1) / 6;
    int x1 = (sum2 - sqrt(2 * squaredSum2 - sum2 * sum2)) / 2;
    int x2 = (sum2 + sqrt(2 * squaredSum2 - sum2 * sum2)) / 2;
    return {x1, x2};
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{0, 1, 1, 0}, {0, 1}},
      {{0, 3, 2, 1, 3, 2}, {2, 3}},
      {{7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2}, {4, 5}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().getSneakyNumbers(nums) == ans);
  }
}
