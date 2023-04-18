/*
 * @Date: 2023-04-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-18
 * @FilePath: /algorithm/cpp/1026_max_ancestor_diff/max_ancestor_diff.CPP
 */

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <vector>

using namespace std;

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
  int dfs(TreeNode *root, int mi, int ma) {
    if (root == nullptr) {
      return 0;
    }
    int diff = max(abs(root->val - mi), abs(root->val - ma));
    mi = min(mi, root->val);
    ma = max(ma, root->val);
    diff = max(diff, dfs(root->left, mi, ma));
    diff = max(diff, dfs(root->right, mi, ma));
    return diff;
  }

  int maxAncestorDiff(TreeNode *root) { return dfs(root, root->val, root->val); }
};

int main() {
  {
    TreeNode *root =
        new TreeNode(8, new TreeNode(3, new TreeNode(1), new TreeNode(6, new TreeNode(4), new TreeNode(7))),
                     new TreeNode(10, nullptr, new TreeNode(14, new TreeNode(13), nullptr)));
    int ans = 7;
    assert(Solution().maxAncestorDiff(root) == ans);
  }

  {
    TreeNode *root = new TreeNode(1, nullptr, new TreeNode(2, nullptr, new TreeNode(0, new TreeNode(3), nullptr)));
    int ans = 3;
    assert(Solution().maxAncestorDiff(root) == ans);
  }
}