/*
 * @Date: 2024-02-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-24
 * @FilePath: /algorithm/cpp/2476_closest_nodes/closest_nodes.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <tuple>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <functional>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> closestNodes(TreeNode *root, vector<int> &queries) {
    vector<int> arr;
    function<void(TreeNode *)> dfs = [&](TreeNode *root) {
      if (!root) {
        return;
      }
      dfs(root->left);
      arr.emplace_back(root->val);
      dfs(root->right);
    };
    dfs(root);

    vector<vector<int>> res;
    for (int val : queries) {
      int maxVal = -1, minVal = -1;
      auto it = lower_bound(arr.begin(), arr.end(), val);
      if (it != arr.end()) {
        maxVal = *it;
        if (*it == val) {
          minVal = *it;
          res.push_back({minVal, maxVal});
          continue;
        }
      }
      if (it != arr.begin()) {
        minVal = *(--it);
      }
      res.push_back({minVal, maxVal});
    }
    return res;
  }
};

int main() {
  vector<tuple<TreeNode *, vector<int>, vector<vector<int>>>> tests{
      {new TreeNode{6, new TreeNode{2, new TreeNode{1}, new TreeNode{4}},
                    new TreeNode{13, new TreeNode{9}, new TreeNode{15, new TreeNode{14}, nullptr}}},
       {2, 5, 16},
       {{2, 2}, {4, 6}, {15, -1}}},
      {new TreeNode{4, nullptr, new TreeNode{9}}, {3}, {{-1, 4}}},
  };

  for (auto &[root, queries, ans] : tests) {
    assert(Solution().closestNodes(root, queries) == ans);
  }
}