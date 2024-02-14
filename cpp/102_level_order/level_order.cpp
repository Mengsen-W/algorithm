/*
 * @Date: 2024-02-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-14
 * @FilePath: /algorithm/cpp/102_level_order/level_order.cpp
 */

//  Definition for a binary tree node.
#include <cassert>
#include <optional>
#include <tuple>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> levelOrder(TreeNode *root) {
    vector<vector<int>> ret;
    if (!root) {
      return ret;
    }

    queue<TreeNode *> q;
    q.push(root);
    while (!q.empty()) {
      int currentLevelSize = q.size();
      ret.push_back(vector<int>());
      for (int i = 1; i <= currentLevelSize; ++i) {
        auto node = q.front();
        q.pop();
        ret.back().push_back(node->val);
        if (node->left) q.push(node->left);
        if (node->right) q.push(node->right);
      }
    }
    return ret;
  }
};

int main() {
  vector<tuple<TreeNode *, vector<vector<int>>>> tests{
      {new TreeNode{3, new TreeNode{9}, new TreeNode{20, new TreeNode{15}, new TreeNode{7}}}, {{3}, {9, 20}, {15, 7}}},
      {new TreeNode{1}, {{1}}},
      {nullptr, {}},
  };

  for (auto &[root, ans] : tests) {
    assert(Solution().levelOrder(root) == ans);
  }
}