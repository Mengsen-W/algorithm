/*
 * @Date: 2021-11-26 01:23:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-26 01:31:55
 */

#include <cassert>

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
  TreeNode *searchBST(TreeNode *root, int val) {
    if (root == nullptr) return nullptr;
    if (val == root->val) return root;
    return searchBST(val < root->val ? root->left : root->right, val);
  }
};

int main() {
  Solution solution;
  TreeNode *root = new TreeNode(
      4, new TreeNode(2, new TreeNode(1), new TreeNode(3)), new TreeNode(7));
  TreeNode *result = solution.searchBST(root, 2);
  assert(result->val == 2);
  assert(result->left->val == 1);
  assert(result->right->val == 3);
}