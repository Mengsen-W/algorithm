/*
 * @Date: 2023-02-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-06
 * @FilePath: /algorithm/cpp/2331_evaluate_tree/evaluate_tree.cpp
 */

#include <cassert>

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
  bool evaluateTree(TreeNode *root) {
    if (root->left == nullptr) {
      return root->val;
    }
    if (root->val == 2) {
      return evaluateTree(root->left) || evaluateTree(root->right);
    } else {
      return evaluateTree(root->left) && evaluateTree(root->right);
    }
  }
};

int main() {
  {
    TreeNode *root = new TreeNode{2, new TreeNode{1}, new TreeNode{3, new TreeNode{0}, new TreeNode{1}}};
    bool ans = true;
    assert(Solution().evaluateTree(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{0};
    bool ans = false;
    assert(Solution().evaluateTree(root) == ans);
  }
}