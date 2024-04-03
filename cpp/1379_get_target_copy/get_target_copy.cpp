/*
 * @Date: 2024-04-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-03
 * @FilePath: /algorithm/cpp/1379_get_target_copy/get_target_copy.cpp
 */

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

#include <queue>

using namespace std;

class Solution {
 public:
  TreeNode *getTargetCopy(TreeNode *original, TreeNode *cloned, TreeNode *target) {
    queue<TreeNode *> q1, q2;
    q1.push(original);
    q2.push(cloned);
    while (!q1.empty()) {
      TreeNode *node1 = q1.front(), *node2 = q2.front();
      q1.pop();
      q2.pop();
      if (node1 == target) {
        return node2;
      }
      if (node1->left != nullptr) {
        q1.push(node1->left);
        q2.push(node2->left);
      }
      if (node1->right != nullptr) {
        q1.push(node1->right);
        q2.push(node2->right);
      }
    }
    return nullptr;  // impossible case
  }
};
