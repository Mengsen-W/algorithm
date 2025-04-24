#include <cassert>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  int countCompleteSubarrays(vector<int>& nums) {
    int res = 0;
    unordered_map<int, int> cnt;
    int n = nums.size();
    int right = 0;
    unordered_set<int> distinct(nums.begin(), nums.end());
    int distinct_count = distinct.size();

    for (int left = 0; left < n; left++) {
      if (left > 0) {
        int remove = nums[left - 1];
        cnt[remove]--;
        if (cnt[remove] == 0) {
          cnt.erase(remove);
        }
      }
      while (right < n && cnt.size() < distinct_count) {
        int add = nums[right];
        cnt[add]++;
        right++;
      }
      if (cnt.size() == distinct_count) {
        res += (n - right + 1);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{1, 3, 1, 2, 2}, 4},
      {{5, 5, 5, 5}, 10},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().countCompleteSubarrays(nums) == ans);
  }
}