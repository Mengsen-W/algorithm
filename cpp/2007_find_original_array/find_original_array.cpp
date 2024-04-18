/*
 * @Date: 2024-04-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-18
 * @FilePath: /algorithm/cpp/2007_find_original_array/find_original_array.cpp
 */

#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> findOriginalArray(vector<int>& changed) {
    sort(changed.begin(), changed.end());
    unordered_map<int, int> count;
    for (int a : changed) {
      count[a]++;
    }
    vector<int> res;
    for (int a : changed) {
      if (count[a] == 0) {
        continue;
      }
      count[a]--;
      if (count[a * 2] == 0) {
        return {};
      }
      count[a * 2]--;
      res.push_back(a);
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{1, 3, 4, 2, 6, 8}, {1, 3, 4}},
      {{6, 3, 0, 1}, {}},
      {{1}, {}},
  };

  for (auto& [changed, ans] : tests) {
    assert(Solution().findOriginalArray(changed) == ans);
  }
}