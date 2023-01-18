/*
 * @Date: 2022-06-02 09:53:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-02 10:43:04
 * @FilePath: /algorithm/450_delete_bst_node/delete_bst_node.cpp
 */

#include <cassert>

//  Definition for a binary tree node.
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
  TreeNode *deleteNode(TreeNode *root, int key) {
    TreeNode *cur = root, *curParent = nullptr;
    while (cur && cur->val != key) {
      curParent = cur;
      if (cur->val > key) {
        cur = cur->left;
      } else {
        cur = cur->right;
      }
    }
    if (!cur) {
      return root;
    }
    if (!cur->left && !cur->right) {
      cur = nullptr;
    } else if (!cur->right) {
      cur = cur->left;
    } else if (!cur->left) {
      cur = cur->right;
    } else {
      TreeNode *successor = cur->right, *successorParent = cur;
      while (successor->left) {
        successorParent = successor;
        successor = successor->left;
      }
      if (successorParent->val == cur->val) {
        successorParent->right = successor->right;
      } else {
        successorParent->left = successor->right;
      }
      successor->right = cur->right;
      successor->left = cur->left;
      cur = successor;
    }
    if (!curParent) {
      return cur;
    } else {
      if (curParent->left && curParent->left->val == key) {
        curParent->left = cur;
      } else {
        curParent->right = cur;
      }
      return root;
    }
  }
};

int main() {
  assert(Solution()
             .deleteNode(new TreeNode(5, new TreeNode(3, new TreeNode(2), new TreeNode(4)),
                                      new TreeNode(6, nullptr, new TreeNode(7))),
                         3)
             ->left->val == 4);

  assert(
      Solution()
          .deleteNode(
              new TreeNode(5, new TreeNode(2, nullptr, new TreeNode(4)), new TreeNode(6, nullptr, new TreeNode(7))), 0)
          ->val == 5);

  assert(Solution().deleteNode(nullptr, 0) == nullptr);
}