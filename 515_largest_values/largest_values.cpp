/*
 * @Date: 2022-06-24
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-24
 * @FilePath: /algorithm/515_largest_values/largest_values.cpp
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
  vector<int> largestValues(TreeNode *root) {
    if (!root) {
      return {};
    }
    vector<int> res;
    queue<TreeNode *> q;
    q.push(root);
    while (!q.empty()) {
      int len = q.size();
      int maxVal = INT_MIN;
      while (len > 0) {
        len--;
        auto t = q.front();
        q.pop();
        maxVal = max(maxVal, t->val);
        if (t->left) {
          q.push(t->left);
        }
        if (t->right) {
          q.push(t->right);
        }
      }
      res.push_back(maxVal);
    }
    return res;
  }
};

int main() {
  {
    TreeNode *root =
        new TreeNode(1, new TreeNode(3, new TreeNode(5), new TreeNode(3)), new TreeNode(2, nullptr, new TreeNode(9)));
    vector<int> ans{1, 3, 9};
    assert(Solution().largestValues(root) == ans);
  }

  {
    TreeNode *root = new TreeNode(1, new TreeNode(2), new TreeNode(3));
    vector<int> ans{1, 3};
    assert(Solution().largestValues(root) == ans);
  }

  return 0;
}