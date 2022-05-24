/*
 * @Date: 2022-05-24 19:50:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-24 19:59:24
 * @FilePath: /algorithm/965_is_unival_tree/is_unival_tree.cpp
 */

#include <cassert>

// Definition for a binary tree node.
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
  bool isUnivalTree(TreeNode *root) {
    if (!root) {
      return true;
    }
    if (root->left) {
      if (root->val != root->left->val || !isUnivalTree(root->left)) {
        return false;
      }
    }
    if (root->right) {
      if (root->val != root->right->val || !isUnivalTree(root->right)) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().isUnivalTree(new TreeNode(1, new TreeNode(1, new TreeNode(1), new TreeNode(1)),
                                              new TreeNode(1, nullptr, new TreeNode(1)))) == true);

  assert(Solution().isUnivalTree(new TreeNode(2, new TreeNode(2, new TreeNode(5), new TreeNode(2)),
                                              new TreeNode(2, nullptr, nullptr))) == false);
}