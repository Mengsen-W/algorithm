/*
 * @Date: 2022-06-19 16:53:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-19 17:12:33
 * @FilePath: /algorithm/508_find_frequent_tree_sum/find_frequent_tree_sum.cpp
 */

#include <cassert>
#include <unordered_map>
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
  unordered_map<int, int> cnt;
  int maxCnt = 0;

  int dfs(TreeNode *node) {
    if (node == nullptr) {
      return 0;
    }
    int sum = node->val + dfs(node->left) + dfs(node->right);
    maxCnt = max(maxCnt, ++cnt[sum]);
    return sum;
  }

 public:
  vector<int> findFrequentTreeSum(TreeNode *root) {
    dfs(root);
    vector<int> ans;
    for (auto &[s, c] : cnt) {
      if (c == maxCnt) {
        ans.emplace_back(s);
      }
    }
    return ans;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode(5, new TreeNode(2), new TreeNode(-3));
    vector<int> ans{4, -3, 2};
    assert((Solution().findFrequentTreeSum(root) == ans));
  }
  {
    TreeNode *root = new TreeNode(5, new TreeNode(2), new TreeNode(-5));
    vector<int> ans{2};
    assert((Solution().findFrequentTreeSum(root) == ans));
  }
}