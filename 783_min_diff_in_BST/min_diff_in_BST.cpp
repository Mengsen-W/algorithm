/*
 * @Date: 2021-04-13 08:41:41
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-13 08:58:21
 */

#include <algorithm>
#include <cassert>
using namespace std;

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right)
      : val(x), left(left), right(right) {}
};

void dfs(TreeNode* root, int& pre, int& ans) {
  if (root == nullptr) {
    return;
  }
  dfs(root->left, pre, ans);
  if (pre == -1) {
    pre = root->val;
  } else {
    ans = min(ans, root->val - pre);
    pre = root->val;
  }
  dfs(root->right, pre, ans);
}
int min_diff_in_BST(TreeNode* root) {
  int ans = INT_MAX, pre = -1;
  dfs(root, pre, ans);
  return ans;
}

int main() {
  {
    TreeNode* root =
        new TreeNode(4,
                     new TreeNode(2, new TreeNode(1, nullptr, nullptr),
                                  new TreeNode(3, nullptr, nullptr)),
                     new TreeNode(6));
    assert(min_diff_in_BST(root) == 1);
  }
  {
    TreeNode* root =
        new TreeNode(1, new TreeNode(0, nullptr, nullptr),
                     new TreeNode(48, new TreeNode(12, nullptr, nullptr),
                                  new TreeNode(49, nullptr, nullptr)));
    assert(min_diff_in_BST(root) == 1);
  }
}