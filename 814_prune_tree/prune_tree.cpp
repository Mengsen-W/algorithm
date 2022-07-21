/*
 * @Date: 2022-07-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-21
 * @FilePath: /algorithm/814_prune_tree/prune_tree.cpp
 */

#include <cassert>
#include <iostream>

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
  TreeNode *pruneTree(TreeNode *root) {
    if (!root) {
      return nullptr;
    }
    root->left = pruneTree(root->left);
    root->right = pruneTree(root->right);
    if (!root->left && !root->right && !root->val) {
      return nullptr;
    }
    return root;
  }
};

int main() {
  Solution().pruneTree(new TreeNode(1, nullptr, new TreeNode(0, new TreeNode(0), new TreeNode(0))));
  Solution().pruneTree(new TreeNode(1, new TreeNode(0, new TreeNode(0), new TreeNode(0)),
                                    new TreeNode(1, new TreeNode(0), new TreeNode(1))));
  Solution().pruneTree(new TreeNode(1, new TreeNode(1, new TreeNode(1, new TreeNode(0), nullptr), new TreeNode(1)),
                                    new TreeNode(0, new TreeNode(1), new TreeNode(0))));
}
