#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int findMin(vector<int>& nums) {
    int low = 0;
    int high = nums.size() - 1;
    while (low < high) {
      int pivot = low + (high - low) / 2;
      if (nums[pivot] < nums[high]) {
        high = pivot;
      } else if (nums[pivot] > nums[high]) {
        low = pivot + 1;
      } else {
        high -= 1;
      }
    }
    return nums[low];
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3, 5}, 1},
      {{2, 2, 2, 0, 1}, 0},
  };

  for (auto& t : tests) {
    assert(Solution().findMin(get<0>(t)) == get<1>(t));
  }
  return 0;
}
