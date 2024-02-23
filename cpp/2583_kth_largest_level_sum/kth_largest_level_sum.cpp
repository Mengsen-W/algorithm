/*
 * @Date: 2024-02-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-23
 * @FilePath: /algorithm/cpp/2583_kth_largest_level_sum/kth_largest_level_sum.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <tuple>
#include <vector>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <queue>

using namespace std;

class Solution {
 public:
  long long kthLargestLevelSum(TreeNode *root, int k) {
    queue<TreeNode *> q;
    q.push(root);
    vector<long long> levelSumArray;
    while (!q.empty()) {
      long long levelSum = 0, size = q.size();
      for (int i = 0; i < size; i++) {
        TreeNode *node = q.front();
        q.pop();
        levelSum += node->val;
        if (node->left) {
          q.push(node->left);
        }
        if (node->right) {
          q.push(node->right);
        }
      }
      levelSumArray.push_back(levelSum);
    }
    if (levelSumArray.size() < k) {
      return -1;
    }
    sort(levelSumArray.begin(), levelSumArray.end());
    return *(levelSumArray.end() - k);
  }
};

int main() {
  vector<tuple<TreeNode *, int, long long>> tests{
      {
          new TreeNode{5, new TreeNode{8, new TreeNode{2, new TreeNode{4}, new TreeNode{6}}, new TreeNode{1}},
                       new TreeNode{9, new TreeNode{3}, new TreeNode{7}}},
          2,
          13,
      },
      {
          new TreeNode{1, new TreeNode{2, new TreeNode{3}, nullptr}, nullptr},
          1,
          3,
      },
  };

  for (auto &[root, k, ans] : tests) {
    assert(Solution().kthLargestLevelSum(root, k) == ans);
  }
}