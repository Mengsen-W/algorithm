#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> relocateMarbles(vector<int>& nums, vector<int>& moveFrom, vector<int>& moveTo) {
    vector<int> ans;
    unordered_map<int, bool> mp;

    for (int i = 0; i < nums.size(); i++) {
      mp[nums[i]] = true;
    }

    for (int i = 0; i < moveFrom.size(); i++) {
      mp.erase(moveFrom[i]);
      mp[moveTo[i]] = true;
    }

    for (const auto& pair : mp) {
      ans.push_back(pair.first);
    }
    sort(ans.begin(), ans.end());
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>, vector<int>>> tests{
      {{1, 6, 7, 8}, {1, 7, 2}, {2, 9, 5}, {5, 6, 8, 9}},
      {{1, 1, 3, 3}, {1, 3}, {2, 2}, {2}},
  };

  for (auto& [nums, moveFrom, moveTo, ans] : tests) {
    assert(Solution().relocateMarbles(nums, moveFrom, moveTo) == ans);
  }
}