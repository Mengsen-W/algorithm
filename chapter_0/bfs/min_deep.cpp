/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-21 19:55:54
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-21 20:11:47
 */

#include <queue>

using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

int min_deep_bfs(TreeNode *root) {
  if (root == nullptr) return 0;
  queue<TreeNode *> que;
  que.push(root);
  int depth = 1;

  while (!que.empty()) {
    int size = que.size();
    for (int i = 0; i < size; ++i) {
      TreeNode *cul = que.front();
      if (cul->left == nullptr && cul->right == nullptr) return depth;
      if (cul->left != nullptr) que.push(cul->left);
      if (cul->right != nullptr) que.push(cul->right);
      que.pop();
    }
    ++depth;
  }
  return depth;
}

int min_deep_dfs(TreeNode *root) {
  if (root == nullptr) return 0;
  int left = min_deep_dfs(root->left);
  int right = min_deep_dfs(root->right);
  if (!left || !right) return left + right + 1;
  return min(left, right) + 1;
}

int main() { return 0; }