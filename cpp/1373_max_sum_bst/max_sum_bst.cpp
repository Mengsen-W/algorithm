/*
 * @Date: 2023-05-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-20
 * @FilePath: /algorithm/cpp/1373_max_sum_bst/max_sum_bst.cpp
 */

#include <cassert>
#include <cmath>
#include <cstdlib>

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
 public:
  static constexpr int inf = 0x3f3f3f3f;
  int res;
  struct SubTree {
    bool isBST;
    int minValue;
    int maxValue;
    int sumValue;
    SubTree(bool isBST, int minValue, int maxValue, int sumValue)
        : isBST(isBST), minValue(minValue), maxValue(maxValue), sumValue(sumValue) {}
  };

  SubTree dfs(TreeNode *root) {
    if (root == nullptr) {
      return SubTree(true, inf, -inf, 0);
    }
    auto left = dfs(root->left);
    auto right = dfs(root->right);

    if (left.isBST && right.isBST && root->val > left.maxValue && root->val < right.minValue) {
      int sum = root->val + left.sumValue + right.sumValue;
      res = fmax(res, sum);
      return SubTree(true, fmin(left.minValue, root->val), fmax(root->val, right.maxValue), sum);
    } else {
      return SubTree(false, 0, 0, 0);
    }
  }

  int maxSumBST(TreeNode *root) {
    res = 0;
    dfs(root);
    return res;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode{1, new TreeNode{4, new TreeNode{2}, new TreeNode{4}},
                                  new TreeNode{3, new TreeNode{2}, new TreeNode{5, new TreeNode{4}, new TreeNode{6}}}};
    int ans = 20;
    assert(Solution().maxSumBST(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{4, new TreeNode{3, new TreeNode{1}, new TreeNode{2}}, nullptr};
    int ans = 2;
    assert(Solution().maxSumBST(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{-4, new TreeNode{-2}, new TreeNode{-5}};
    int ans = 0;
    assert(Solution().maxSumBST(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{2, new TreeNode{1}, new TreeNode{3}};
    int ans = 6;
    assert(Solution().maxSumBST(root) == ans);
  }

  {
    TreeNode *root =
        new TreeNode{5, new TreeNode{4, new TreeNode{3}, nullptr}, new TreeNode{8, new TreeNode{6}, new TreeNode{3}}};
    int ans = 7;
    assert(Solution().maxSumBST(root) == ans);
  }
}