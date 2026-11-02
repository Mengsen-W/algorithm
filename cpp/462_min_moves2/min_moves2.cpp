#include <cassert>
#include <cstdlib>
#include <ctime>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int quickSelect(vector<int>& nums, int left, int right, int index) {
    int q = randomPartition(nums, left, right);
    if (q == index) {
      return nums[q];
    } else {
      return q < index ? quickSelect(nums, q + 1, right, index) : quickSelect(nums, left, q - 1, index);
    }
  }

  inline int randomPartition(vector<int>& nums, int left, int right) {
    int i = rand() % (right - left + 1) + left;
    swap(nums[i], nums[right]);
    return partition(nums, left, right);
  }

  inline int partition(vector<int>& nums, int left, int right) {
    int x = nums[right], i = left - 1;
    for (int j = left; j < right; ++j) {
      if (nums[j] <= x) {
        swap(nums[++i], nums[j]);
      }
    }
    swap(nums[i + 1], nums[right]);
    return i + 1;
  }

  int minMoves2(vector<int> nums) {
    srand(time(0));
    int n = nums.size(), x = quickSelect(nums, 0, n - 1, n / 2), ret = 0;
    for (int i = 0; i < n; ++i) {
      ret += abs(nums[i] - x);
    }
    return ret;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 2, 3}, 2},
      {{1, 10, 2, 9}, 16},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().minMoves2(nums) == expected);
  }
}