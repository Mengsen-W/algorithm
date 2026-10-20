#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  int sortableIntegers(vector<int>& nums) {
    int n = nums.size();
    vector<int> fac;

    for (int x = 1; x * x <= n; ++x) {
      if (n % x == 0) {
        fac.push_back(x);
        if (n / x != x) {
          fac.push_back(n / x);
        }
      }
    }

    vector<int> target = nums;
    ranges::sort(target);

    // 检查能否通过对 nums1 进行若干次循环移动操作得到 nums2
    auto check = [](const vector<int>& nums1, const vector<int>& nums2) {
      int L = nums1.size();

      for (int i = 0; i < L - 1; ++i) {
        if (nums1[i] > nums1[i + 1]) {
          vector<int> rotated;
          rotated.insert(rotated.end(), nums1.begin() + i + 1, nums1.end());
          rotated.insert(rotated.end(), nums1.begin(), nums1.begin() + i + 1);
          return rotated == nums2;
        }
      }

      return nums1 == nums2;
    };

    int ans = 0;

    for (int d : fac) {
      bool allGood = true;
      for (int left = 0; left < n; left += d) {
        int right = left + d - 1;

        vector<int> sub1(nums.begin() + left, nums.begin() + right + 1);
        vector<int> sub2(target.begin() + left, target.begin() + right + 1);

        if (!check(sub1, sub2)) {
          allGood = false;
          break;
        }
      }
      if (allGood) {
        ans += d;
      }
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{3, 1, 2}, 3},
      {{7, 6, 5}, 0},
      {{5, 8}, 3},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().sortableIntegers(nums) == expected);
  }
}