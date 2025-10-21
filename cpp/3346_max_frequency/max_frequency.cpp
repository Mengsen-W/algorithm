#include <algorithm>
#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxFrequency(vector<int>& nums, int k, int numOperations) {
    sort(nums.begin(), nums.end());

    int ans = 0;
    unordered_map<int, int> numCount;

    int lastNumIndex = 0;
    for (int i = 0; i < nums.size(); ++i) {
      if (nums[i] != nums[lastNumIndex]) {
        numCount[nums[lastNumIndex]] = i - lastNumIndex;
        ans = max(ans, i - lastNumIndex);
        lastNumIndex = i;
      }
    }

    numCount[nums[lastNumIndex]] = nums.size() - lastNumIndex;
    ans = max(ans, (int)nums.size() - lastNumIndex);

    auto leftBound = [&](int value) {
      int left = 0, right = nums.size() - 1;
      while (left < right) {
        int mid = (left + right) / 2;
        if (nums[mid] < value) {
          left = mid + 1;
        } else {
          right = mid;
        }
      }
      return left;
    };

    auto rightBound = [&](int value) {
      int left = 0, right = nums.size() - 1;
      while (left < right) {
        int mid = (left + right + 1) / 2;
        if (nums[mid] > value) {
          right = mid - 1;
        } else {
          left = mid;
        }
      }
      return left;
    };

    for (int i = nums.front(); i <= nums.back(); i++) {
      int l = leftBound(i - k);
      int r = rightBound(i + k);

      int tempAns;
      if (numCount.count(i)) {
        tempAns = min(r - l + 1, numCount[i] + numOperations);
      } else {
        tempAns = min(r - l + 1, numOperations);
      }
      ans = max(ans, tempAns);
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int, int>> tests{
      {{1, 4, 5}, 1, 2, 2},
      {{5, 11, 20, 20}, 5, 1, 2},
  };

  for (auto& [nums, k, numOperations, expect] : tests) {
    assert(Solution().maxFrequency(nums, k, numOperations) == expect);
  }
}