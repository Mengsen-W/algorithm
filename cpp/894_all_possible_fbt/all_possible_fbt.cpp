/*
 * @Date: 2024-04-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-02
 * @FilePath: /algorithm/cpp/894_all_possible_fbt/all_possible_fbt.cpp
 */

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <vector>

using namespace std;

class Solution {
 public:
  vector<TreeNode *> allPossibleFBT(int n) {
    if (n % 2 == 0) {
      return {};
    }

    vector<vector<TreeNode *>> dp(n + 1);
    dp[1] = {new TreeNode(0)};
    for (int i = 3; i <= n; i += 2) {
      for (int j = 1; j < i; j += 2) {
        for (TreeNode *leftSubtree : dp[j]) {
          for (TreeNode *rightSubtrees : dp[i - 1 - j]) {
            TreeNode *root = new TreeNode(0, leftSubtree, rightSubtrees);
            dp[i].emplace_back(root);
          }
        }
      }
    }
    return dp[n];
  }
};

