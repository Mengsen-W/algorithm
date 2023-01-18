/*
 * @Date: 2021-11-18 00:18:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-18 00:26:26
 */

#include <cassert>
#include <cmath>

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
  int ans = 0;

  int findTilt(TreeNode *root) {
    dfs(root);
    return ans;
  }

  int dfs(TreeNode *node) {
    if (node == nullptr) return 0;

    int sumLeft = dfs(node->left);
    int sumRight = dfs(node->right);
    ans += abs(sumLeft - sumRight);
    return sumLeft + sumRight + node->val;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode(1, new TreeNode(2), new TreeNode(3));
    assert(Solution().findTilt(root) == 1);
  }
  {
    TreeNode *root =
        new TreeNode(4, new TreeNode(2, new TreeNode(3), new TreeNode(5)),
                     new TreeNode(9, nullptr, new TreeNode(7)));
    assert(Solution().findTilt(root) == 15);
  }
  {
    TreeNode *root = new TreeNode(
        21,
        new TreeNode(7, new TreeNode(1),
                     new TreeNode(1, new TreeNode(3), new TreeNode(3))),
        new TreeNode(14, new TreeNode(2), new TreeNode(2)));
    assert(Solution().findTilt(root) == 9);
  }
}