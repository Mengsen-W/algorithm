/*
 * @Date: 2022-07-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-31
 * @FilePath: /algorithm/1161_max_level_sum/max_level_sum.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

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
  int maxLevelSum(TreeNode *root) {
    int ans = 1, maxSum = root->val;
    vector<TreeNode *> q = {root};
    for (int level = 1; !q.empty(); ++level) {
      vector<TreeNode *> nq;
      int sum = 0;
      for (auto node : q) {
        sum += node->val;
        if (node->left) {
          nq.emplace_back(node->left);
        }
        if (node->right) {
          nq.emplace_back(node->right);
        }
      }
      if (sum > maxSum) {
        maxSum = sum;
        ans = level;
      }
      q = move(nq);
    }
    return ans;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode(1, new TreeNode(7, new TreeNode(7), new TreeNode(-8)), new TreeNode(0));
    assert(Solution().maxLevelSum(root) == 2);
  }
  {
    TreeNode *root = new TreeNode(
        989, nullptr, new TreeNode(10250, new TreeNode(98693), new TreeNode(-89388, nullptr, new TreeNode(-32127))));
    assert(Solution().maxLevelSum(root) == 2);
  }
}