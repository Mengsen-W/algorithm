#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> minBitwiseArray(vector<int>& nums) {
        for (int& x : nums) { // 注意这里是引用
            if (x == 2) {
                x = -1;
            } else {
                x ^= ((x + 1) & ~x) >> 1;
            }
        }
        return nums;
    }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{2, 3, 5, 7}, {-1, 1, 4, 3}},
      {{11, 13, 31}, {9, 12, 15}},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().minBitwiseArray(nums) == expected);
  }
}