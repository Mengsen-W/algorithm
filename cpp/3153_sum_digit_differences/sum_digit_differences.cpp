#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long sumDigitDifferences(vector<int>& nums) {
    long long res = 0;
    int n = nums.size();
    while (nums[0] > 0) {
      vector<int> cnt(10);
      for (int i = 0; i < n; i++) {
        cnt[nums[i] % 10]++;
        nums[i] /= 10;
      }
      for (int i = 0; i < 10; i++) {
        res += (long long)(n - cnt[i]) * cnt[i];
      }
    }
    return res / 2;
  }
};

int main() {
  vector<tuple<vector<int>, long long>> tests{
      {{13, 23, 12}, 4},
      {{10, 10, 10, 10}, 0},
  };
  
  for (auto &[nums, ans] : tests) {
    assert(Solution().sumDigitDifferences(nums) == ans);
  }
}