/*
 * @Date: 2022-09-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-10
 * @FilePath: /algorithm/669_trim_bst/trim_bst.cpp
 */

#include <assert.h>

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
  TreeNode *trimBST(TreeNode *root, int low, int high) {
    while (root && (root->val < low || root->val > high)) {
      if (root->val < low) {
        root = root->right;
      } else {
        root = root->left;
      }
    }
    if (root == nullptr) {
      return nullptr;
    }
    for (auto node = root; node->left;) {
      if (node->left->val < low) {
        node->left = node->left->right;
      } else {
        node = node->left;
      }
    }
    for (auto node = root; node->right;) {
      if (node->right->val > high) {
        node->right = node->right->left;
      } else {
        node = node->right;
      }
    }
    return root;
  }
};

bool isSameTree(TreeNode *p, TreeNode *q) {
  if (p == nullptr || q == nullptr) return p == q;
  if (p->val != q->val) return false;
  return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
}

int main() {
  {
    TreeNode *root = new TreeNode{1, new TreeNode{0}, new TreeNode{2}};
    int low = 1;
    int high = 2;
    TreeNode *ans = new TreeNode{1, nullptr, new TreeNode{2}};
    assert(isSameTree(Solution().trimBST(root, low, high), ans));
  }

  {
    TreeNode *root =
        new TreeNode{3, new TreeNode{0, nullptr, new TreeNode{2, new TreeNode{1}, nullptr}}, new TreeNode{4}};
    int low = 1;
    int high = 3;
    TreeNode *ans = new TreeNode{3, new TreeNode{2, new TreeNode{1}, nullptr}, nullptr};
    assert(isSameTree(Solution().trimBST(root, low, high), ans));
  }
}