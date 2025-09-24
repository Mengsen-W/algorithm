#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  static constexpr int P = int(1E9) + 7;
  static constexpr int MAX_N = int(1E5) + 5;

  int f[MAX_N];

  void pretreatment() {
    f[0] = 1;
    for (int i = 1; i < MAX_N; ++i) {
      f[i] = (long long)f[i - 1] * 2 % P;
    }
  }

  int numSubseq(vector<int>& nums, int target) {
    pretreatment();

    sort(nums.begin(), nums.end());

    int ans = 0;
    for (int i = 0; i < nums.size() && nums[i] * 2 <= target; ++i) {
      int maxValue = target - nums[i];
      int pos = upper_bound(nums.begin(), nums.end(), maxValue) - nums.begin() - 1;
      int contribute = (pos >= i) ? f[pos - i] : 0;
      ans = (ans + contribute) % P;
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, int>> tests{
      {{3, 5, 6, 7}, 9, 4},
      {{3, 3, 6, 8}, 10, 6},
      {{2, 3, 3, 4, 6, 7}, 12, 61},
  };

  for (auto &[nums, target, expected] : tests) {
    assert(expected == Solution().numSubseq(nums, target));
  }
}