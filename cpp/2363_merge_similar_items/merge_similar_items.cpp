/*
 * @Date: 2023-02-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-28
 * @FilePath: /algorithm/cpp/2363_merge_similar_items/merge_similar_items.cpp
 */

#include <cassert>
#include <map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> mergeSimilarItems(vector<vector<int>> &items1, vector<vector<int>> &items2) {
    map<int, int> mp;
    for (auto &v : items1) {
      mp[v[0]] += v[1];
    }
    for (auto &v : items2) {
      mp[v[0]] += v[1];
    }

    vector<vector<int>> res;
    for (auto &[k, v] : mp) {
      res.push_back({k, v});
    }
    return res;
  }
};

int main() {
  {
    vector<vector<int>> items1{{1, 1}, {4, 5}, {3, 8}};
    vector<vector<int>> items2{{3, 1}, {1, 5}};
    vector<vector<int>> ans{{1, 6}, {3, 9}, {4, 5}};
    assert(Solution().mergeSimilarItems(items1, items2) == ans);
  }

  {
    vector<vector<int>> items1{{1, 1}, {3, 2}, {2, 3}};
    vector<vector<int>> items2{{2, 1}, {3, 2}, {1, 3}};
    vector<vector<int>> ans{{1, 4}, {2, 4}, {3, 4}};
    assert(Solution().mergeSimilarItems(items1, items2) == ans);
  }

  {
    vector<vector<int>> items1{{1, 3}, {2, 2}};
    vector<vector<int>> items2{{7, 1}, {2, 2}, {1, 4}};
    vector<vector<int>> ans{{1, 7}, {2, 4}, {7, 1}};
    assert(Solution().mergeSimilarItems(items1, items2) == ans);
  }
}