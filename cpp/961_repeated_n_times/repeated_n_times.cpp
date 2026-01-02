#include <cassert>
#include <random>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int repeatedNTimes(vector<int> nums) {
    int n = nums.size();
    mt19937 gen{random_device{}()};
    uniform_int_distribution<int> dis(0, n - 1);

    while (true) {
      int x = dis(gen), y = dis(gen);
      if (x != y && nums[x] == nums[y]) {
        return nums[x];
      }
    }
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 3},
      {{2, 1, 2, 5, 3, 2}, 2},
      {{5, 1, 5, 2, 5, 3, 5, 4}, 5},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().repeatedNTimes(nums) == ans);
  }
}
