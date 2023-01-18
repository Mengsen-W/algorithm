/*
 * @Date: 2022-09-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-02
 * @FilePath: /algorithm/687_longest_univalue_path/longest_univalue_path.cpp
 */

#include <algorithm>
#include <cassert>

using namespace std;

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
 private:
  int res;

 public:
  int longestUnivaluePath(TreeNode *root) {
    res = 0;
    dfs(root);
    return res;
  }

  int dfs(TreeNode *root) {
    if (root == nullptr) {
      return 0;
    }
    int left = dfs(root->left), right = dfs(root->right);
    int left1 = 0, right1 = 0;
    if (root->left && root->left->val == root->val) {
      left1 = left + 1;
    }
    if (root->right && root->right->val == root->val) {
      right1 = right + 1;
    }
    res = max(res, left1 + right1);
    return max(left1, right1);
  }
};

int main() {
  {
    TreeNode *root =
        new TreeNode{5, new TreeNode{4, new TreeNode{1}, new TreeNode{1}}, new TreeNode{5, nullptr, new TreeNode{5}}};
    int ans = 2;
    assert(Solution().longestUnivaluePath(root) == ans);
  }

  {
    TreeNode *root =
        new TreeNode{1, new TreeNode{4, new TreeNode{4}, new TreeNode{4}}, new TreeNode{5, nullptr, new TreeNode{5}}};
    int ans = 2;
    assert(Solution().longestUnivaluePath(root) == ans);
  }
}