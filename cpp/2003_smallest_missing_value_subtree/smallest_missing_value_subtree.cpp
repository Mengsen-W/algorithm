/*
 * @Date: 2023-10-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-31
 * @FilePath: /algorithm/cpp/2003_smallest_missing_value_subtree/smallest_missing_value_subtree.cpp
 */

#include <cassert>
#include <functional>
#include <tuple>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> smallestMissingValueSubtree(vector<int>& parents, vector<int>& nums) {
    int n = parents.size();
    vector<vector<int>> children(n);
    for (int i = 1; i < n; i++) {
      children[parents[i]].push_back(i);
    }

    unordered_set<int> geneSet;
    vector<int> visited(n, 0);
    function<void(int)> dfs = [&](int node) {
      if (visited[node]) {
        return;
      }
      visited[node] = 1;
      geneSet.insert(nums[node]);
      for (auto child : children[node]) {
        dfs(child);
      }
    };

    vector<int> res(n, 1);
    int iNode = 1, node = -1;
    for (int i = 0; i < n; i++) {
      if (nums[i] == 1) {
        node = i;
        break;
      }
    }
    while (node != -1) {
      dfs(node);
      while (geneSet.count(iNode)) {
        iNode++;
      }
      res[node] = iNode;
      node = parents[node];
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, vector<int>>> tests{
      {{-1, 0, 0, 2}, {1, 2, 3, 4}, {5, 1, 1, 1}},
      {{-1, 0, 1, 0, 3, 3}, {5, 4, 6, 2, 1, 3}, {7, 1, 1, 4, 2, 1}},
      {{-1, 2, 3, 0, 2, 4, 1}, {2, 3, 4, 5, 6, 7, 8}, {1, 1, 1, 1, 1, 1, 1}},
  };

  for (auto& [parents, nums, ans] : tests) {
    assert(Solution().smallestMissingValueSubtree(parents, nums) == ans);
  }
}