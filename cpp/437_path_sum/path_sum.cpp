/*
 * @Date: 2021-09-28 09:55:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-28 10:19:34
 */

#include <cassert>
#include <unordered_map>

using namespace std;

// Definition for a binary tree node.
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
  unordered_map<int, int> prefix;

  int dfs(TreeNode *root, long long curr, int targetSum) {
    if (!root) return 0;

    int ret = 0;
    curr += root->val;
    if (prefix.count(curr - targetSum)) ret = prefix[curr - targetSum];

    prefix[curr]++;
    ret += dfs(root->left, curr, targetSum);
    ret += dfs(root->right, curr, targetSum);
    prefix[curr]--;

    return ret;
  }

  int pathSum(TreeNode *root, int targetSum) {
    prefix[0] = 1;
    return dfs(root, 0, targetSum);
  }
};

int main() {
  {
    TreeNode *root = new TreeNode{
        10,
        new TreeNode{5, new TreeNode{3, new TreeNode{3}, new TreeNode{-2}},
                     new TreeNode{2, nullptr, new TreeNode{1}}},
        new TreeNode{-3, nullptr, new TreeNode{11}}};
    assert(Solution().pathSum(root, 8) == 3);
  }
  {
    TreeNode *root = new TreeNode{
        5,
        new TreeNode{4, new TreeNode{11, new TreeNode{7}, new TreeNode{2}},
                     nullptr},
        new TreeNode{8, new TreeNode{13},
                     new TreeNode{4, new TreeNode{5}, new TreeNode{1}}}};
    assert(Solution().pathSum(root, 22) == 3);
  }
}