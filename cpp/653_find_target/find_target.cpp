/*
 * @Date: 2022-03-21 00:26:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-21 01:09:41
 * @FilePath: /algorithm/653_find_target/find_target.cpp
 */

#include <cassert>
#include <stack>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class Solution {
 public:
  TreeNode *getLeft(stack<TreeNode *> &stk) {
    TreeNode *root = stk.top();
    stk.pop();
    TreeNode *node = root->right;
    while (node != nullptr) {
      stk.push(node);
      node = node->left;
    }
    return root;
  }

  TreeNode *getRight(stack<TreeNode *> &stk) {
    TreeNode *root = stk.top();
    stk.pop();
    TreeNode *node = root->left;
    while (node != nullptr) {
      stk.push(node);
      node = node->right;
    }
    return root;
  }

  bool findTarget(TreeNode *root, int k) {
    TreeNode *left = root, *right = root;
    stack<TreeNode *> leftStack, rightStack;
    leftStack.push(left);
    while (left->left != nullptr) {
      leftStack.push(left->left);
      left = left->left;
    }
    rightStack.push(right);
    while (right->right != nullptr) {
      rightStack.push(right->right);
      right = right->right;
    }
    while (left != right) {
      if (left->val + right->val == k) {
        return true;
      }
      if (left->val + right->val < k) {
        left = getLeft(leftStack);
      } else {
        right = getRight(rightStack);
      }
    }
    return false;
  }
};

int main() {
  TreeNode *root =
      new TreeNode(5, new TreeNode(3, new TreeNode(2), new TreeNode(4)),
                   new TreeNode(6, nullptr, new TreeNode(7)));
  assert(Solution().findTarget(root, 9) == true);
  assert(Solution().findTarget(root, 28) == false);
}