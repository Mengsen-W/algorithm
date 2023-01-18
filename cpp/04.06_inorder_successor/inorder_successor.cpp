/*
 * @Date: 2022-05-16 09:45:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-16 09:52:34
 * @FilePath: /algorithm/04.06_inorder_successor/inorder_successor.cpp
 */

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  TreeNode* inorderSuccessor(TreeNode* root, TreeNode* p) {
    TreeNode* successor = nullptr;
    if (p->right != nullptr) {
      successor = p->right;
      while (successor->left != nullptr) {
        successor = successor->left;
      }
      return successor;
    }
    TreeNode* node = root;
    while (node != nullptr) {
      if (node->val > p->val) {
        successor = node;
        node = node->left;
      } else {
        node = node->right;
      }
    }
    return successor;
  }
};

#include <cassert>
int main() {
  {
    TreeNode* root = new TreeNode(2, new TreeNode(1), new TreeNode(3));
    assert(Solution().inorderSuccessor(root, root->left) == root);
  }

  {
    TreeNode* root =
        new TreeNode(5, new TreeNode(3, new TreeNode(2, new TreeNode(1), nullptr), new TreeNode(4)), new TreeNode(6));
    assert(Solution().inorderSuccessor(root, root->right) == nullptr);
  }
}