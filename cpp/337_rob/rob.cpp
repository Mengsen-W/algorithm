/*
 * @Date: 2023-09-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-18
 * @FilePath: /algorithm/cpp/337_rob/rob.cpp
 */

#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

struct SubtreeStatus {
  int selected;
  int notSelected;
};

class Solution {
 public:
  SubtreeStatus dfs(TreeNode *node) {
    if (!node) {
      return {0, 0};
    }
    auto l = dfs(node->left);
    auto r = dfs(node->right);
    int selected = node->val + l.notSelected + r.notSelected;
    int notSelected = max(l.selected, l.notSelected) + max(r.selected, r.notSelected);
    return {selected, notSelected};
  }

  int rob(TreeNode *root) {
    auto rootStatus = dfs(root);
    return max(rootStatus.selected, rootStatus.notSelected);
  }
};

int main() {
  vector<tuple<TreeNode *, int>> tests{
      {
          new TreeNode(3, new TreeNode(2, new TreeNode(3), nullptr), new TreeNode(3, nullptr, new TreeNode(1))),
          7,
      },
      {
          new TreeNode(3, new TreeNode(4, new TreeNode(1), new TreeNode(3)), new TreeNode(5, nullptr, new TreeNode(1))),
          9,
      },
  };

  for (auto &[node, ans] : tests) {
    assert(Solution().rob(node) == ans);
  }
}