#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> findMatrix(vector<int>& nums) {
    unordered_map<int, int> cnt;
    for (int x : nums) {
      cnt[x]++;
    }

    vector<vector<int>> ans;
    while (!cnt.empty()) {
      vector<int> arr;
      for (auto it = cnt.begin(); it != cnt.end();) {
        it->second -= 1;
        arr.emplace_back(it->first);
        if (it->second == 0) {
          it = cnt.erase(it);
        } else {
          it++;
        }
      }
      ans.push_back(arr);
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>>> tests{
      {{1, 3, 4, 1, 2, 3, 1}, {{1, 3, 4, 2}, {1, 3}, {1}}},
      {{1, 2, 3, 4}, {{4, 3, 2, 1}}},
  };

  for (auto& [nums, ans] : tests) {
    assert(Solution().findMatrix(nums) == ans);
  }
}
