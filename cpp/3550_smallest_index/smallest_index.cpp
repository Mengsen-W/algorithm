#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    int smallestIndex(const std::vector<int>& nums) {
        auto getDigitSum = [](int num) {
            int sum = 0;

            while (num > 0) {
                sum += num % 10;
                num /= 10;
            }

            return sum;
        };

        for (int i = 0; i < static_cast<int>(nums.size()); ++i) {
            if (getDigitSum(nums[i]) == i) {
                return i;
            }
        }

        return -1;
    }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3, 2}, 2},
      {{1, 10, 11}, 1},
      {{1, 2, 3}, -1},
  };

  for (auto& [nums, expected] : tests) {
    assert(Solution().smallestIndex(nums) == expected);
  }

  return 0;
}