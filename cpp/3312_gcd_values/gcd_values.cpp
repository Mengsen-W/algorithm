#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> gcdValues(vector<int>& nums, vector<long long>& queries) {
    int m = *max_element(nums.begin(), nums.end());
    vector<long long> cnt(m + 1);
    for (int num : nums) {
      cnt[num]++;
    }
    for (int i = 1; i <= m; i++) {
      for (int j = i * 2; j <= m; j += i) {
        cnt[i] += cnt[j];
      }
    }
    for (int i = 1; i <= m; i++) {
      cnt[i] = cnt[i] * (cnt[i] - 1) / 2;
    }
    for (int i = m; i >= 1; i--) {
      for (int j = i * 2; j <= m; j += i) {
        cnt[i] -= cnt[j];
      }
    }
    for (int i = 1; i <= m; i++) {
      cnt[i] += cnt[i - 1];
    }
    vector<int> ans;
    for (long long q : queries) {
      q++;
      int pos = lower_bound(cnt.begin(), cnt.end(), q) - cnt.begin();
      ans.push_back(pos);
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<long long>, vector<int>>> tests{
      {{2, 3, 4}, {0, 2, 2}, {1, 2, 2}},
      {{4, 4, 2, 1}, {5, 3, 1, 0}, {4, 2, 1, 1}},
      {{2, 2}, {0, 0}, {2, 2}},
  };

  for (auto [nums, queries, expected] : tests) {
    assert(Solution().gcdValues(nums, queries) == expected);
  }
}