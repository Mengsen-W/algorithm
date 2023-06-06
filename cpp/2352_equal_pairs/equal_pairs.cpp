/*
 * @Date: 2023-06-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-06
 * @FilePath: /algorithm/cpp/2352_equal_pairs/equal_pairs.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int equalPairs(vector<vector<int>>& grid) {
    struct hashfunc {
      size_t operator()(const vector<int>& nums) const {
        size_t num = hash<int>{}(nums[0]);
        for (size_t i = 1; i < nums.size(); i++) {
          num ^= hash<int>{}(nums[i]);
        }
        return num;
      }
    };
    int n = grid.size();
    unordered_map<vector<int>, int, hashfunc> cnt;
    for (auto row : grid) {
      cnt[row]++;
    }

    int res = 0;
    for (int j = 0; j < n; j++) {
      vector<int> arr;
      for (int i = 0; i < n; i++) {
        arr.emplace_back(grid[i][j]);
      }
      if (cnt.find(arr) != cnt.end()) {
        res += cnt[arr];
      }
    }
    return res;
  }
};

int main() {
  {
    vector<vector<int>> grid = {{3, 2, 1}, {1, 7, 6}, {2, 7, 7}};
    int ans = 1;
    assert(Solution().equalPairs(grid) == ans);
  }

  {
    vector<vector<int>> grid = {{3, 1, 2, 2}, {1, 4, 4, 5}, {2, 4, 2, 2}, {2, 4, 2, 2}};
    int ans = 3;
    assert(Solution().equalPairs(grid) == ans);
  }
}