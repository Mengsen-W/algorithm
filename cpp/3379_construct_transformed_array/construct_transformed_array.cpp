#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> constructTransformedArray(vector<int>& nums) {
    int n = nums.size();
    vector<int> res(n);
    for (int i = 0; i < n; i++) {
      res[i] = nums[((i + nums[i]) % n + n) % n];
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{3, -2, 1, 1}, {1, 1, 1, 3}},
      {{-1, 4, -1}, {-1, -1, 4}},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().constructTransformedArray(nums) == expected);
  }
}