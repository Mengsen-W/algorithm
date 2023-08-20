/*
 * @Date: 2023-08-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-20
 * @FilePath: /algorithm/cpp/2236_check_tree/check_tree.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <tuple>
#include <vector>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  bool checkTree(TreeNode *root) { return root->val == root->left->val + root->right->val; }
};

int main() {
  std::vector<std::tuple<TreeNode *, bool>> tests{
      {new TreeNode{10, new TreeNode{4}, new TreeNode(6)}, true},
      {new TreeNode{5, new TreeNode{3}, new TreeNode(1)}, false},
  };

  for (auto &[root, expected] : tests) {
    assert(Solution().checkTree(root) == expected);
  }
}
