/*
 * @Date: 2021-04-27 08:38:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-27 09:09:24
 */

#include <cassert>
#include <queue>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

int range_sum_bst_dfs(TreeNode *root, int low, int high) {
  if (root == nullptr) return 0;
  if (root->val >= low && root->val <= high)
    return root->val + range_sum_bst_dfs(root->left, low, high) +
           range_sum_bst_dfs(root->right, low, high);
  else if (root->val < low)
    return range_sum_bst_dfs(root->right, low, high);
  else
    return range_sum_bst_dfs(root->left, low, high);
}

int range_sum_bst_bfs(TreeNode *root, int low, int high) {
  int val = 0;
  std::queue<TreeNode *> q({root});
  while (!q.empty()) {
    TreeNode *node = q.front();
    q.pop();
    if (node == nullptr) continue;
    if (node->val > high) {
      q.push(node->left);
    } else if (node->val < low) {
      q.push(node->right);
    } else {
      val += node->val;
      q.push(node->left);
      q.push(node->right);
    }
  }
  return val;
}

int main() {
  {
    TreeNode *root = new TreeNode{
        10,
        new TreeNode{5, new TreeNode{3, nullptr, nullptr},
                     new TreeNode{7, nullptr, nullptr}},
        new TreeNode{15, nullptr, new TreeNode{18, nullptr, nullptr}}};
    int low = 7, high = 15;
    assert(range_sum_bst_dfs(root, low, high) == 32);
  }
  {
    TreeNode *root = new TreeNode{
        10,
        new TreeNode{
            5, new TreeNode{3, new TreeNode{3, nullptr, nullptr}, nullptr},
            new TreeNode{7, new TreeNode{6, nullptr, nullptr}, nullptr}},
        new TreeNode{15, new TreeNode{13, nullptr, nullptr},
                     new TreeNode{18, nullptr, nullptr}}};
    int low = 6, high = 10;
    assert(range_sum_bst_bfs(root, low, high) == 23);
  }
}
