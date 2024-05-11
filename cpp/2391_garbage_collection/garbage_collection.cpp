/*
 * @Date: 2024-05-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-11
 * @FilePath: /algorithm/cpp/2391_garbage_collection/garbage_collection.cpp
 */

#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int garbageCollection(vector<string>& garbage, vector<int>& travel) {
    unordered_map<char, int> distance;
    int res = 0, cur_dis = 0;
    for (int i = 0; i < garbage.size(); i++) {
      res += garbage[i].size();
      if (i > 0) {
        cur_dis += travel[i - 1];
      }
      for (auto c : garbage[i]) {
        distance[c] = cur_dis;
      }
    }
    for (auto& [k, v] : distance) {
      res += v;
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<string>, vector<int>, int>> tests{
      {{"G", "P", "GP", "GG"}, {2, 4, 3}, 21},
      {{"MMM", "PGM", "GP"}, {3, 10}, 37},
  };

  for (auto& [garbage, travel, ans] : tests) {
    assert(Solution().garbageCollection(garbage, travel) == ans);
  }
}
