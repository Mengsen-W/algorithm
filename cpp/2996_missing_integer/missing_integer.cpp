#include <algorithm>
#include <cassert>
#include <iostream>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int missingInteger(std::vector<int>& nums) {
    int n = nums.size();
    std::unordered_set<int> num_set(nums.begin(), nums.end());
    int prefix_len = 1;

    for (int i = 1; i < n; i++) {
      if (nums[i] == nums[i - 1] + 1) {
        prefix_len += 1;
      } else {
        break;
      }
    }

    int total = (nums[prefix_len - 1] + nums[0]) * prefix_len / 2;
    while (num_set.count(total)) {
      total += 1;
    }

    return total;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3, 2, 5}, 6},
      {{3, 4, 5, 1, 12, 14, 13}, 15},
      {{38}, 38},
      {{1,  49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26,
        25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9,  8,  7,  6,  5,  4,  3,  2},
       50}};

  for (auto& [input, expected] : tests) {
    std::cout << Solution().missingInteger(input) << std::endl;
  }
}