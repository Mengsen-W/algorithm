#include <cassert>
#include <numeric>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<long long> distance(vector<int>& nums) {
    int n = nums.size();
    unordered_map<int, vector<int>> groups;
    for (int i = 0; i < n; i++) {
      groups[nums[i]].push_back(i);
    }
    vector<long long> res(n);
    for (const auto& p : groups) {
      const auto& group = p.second;
      long long total = accumulate(group.begin(), group.end(), 0LL);
      long long prefixTotal = 0;
      for (int i = 0; i < group.size(); i++) {
        res[group[i]] = total - prefixTotal * 2 + group[i] * (2 * i - group.size());
        prefixTotal += group[i];
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<long long>>> tests{
      {{1, 3, 1, 1, 2}, {5, 0, 3, 4, 0}},
      {{0, 5, 3}, {0, 0, 0}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().distance(nums) == ans);
  }
}
