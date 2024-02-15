/*
 * @Date: 2024-02-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-15
 * @FilePath: /algorithm/cpp/107_level_order_bottom/level_order_bottom.cpp
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
  vector<vector<int>> levelOrderBottom(TreeNode *root) {
    auto levelOrder = vector<vector<int>>();
    if (!root) {
      return levelOrder;
    }
    queue<TreeNode *> q;
    q.push(root);
    while (!q.empty()) {
      auto level = vector<int>();
      int size = q.size();
      for (int i = 0; i < size; ++i) {
        auto node = q.front();
        q.pop();
        level.push_back(node->val);
        if (node->left) {
          q.push(node->left);
        }
        if (node->right) {
          q.push(node->right);
        }
      }
      levelOrder.push_back(level);
    }
    reverse(levelOrder.begin(), levelOrder.end());
    return levelOrder;
  }
};

int main() {
  vector<tuple<TreeNode *, vector<vector<int>>>> tests{
      {new TreeNode{3, new TreeNode{9}, new TreeNode{20, new TreeNode{15}, new TreeNode{7}}}, {{15, 7}, {9, 20}, {3}}},
      {new TreeNode{1}, {{1}}},
      {nullptr, {}},
  };

  for (auto &[root, ans] : tests) {
    assert(Solution().levelOrderBottom(root) == ans);
  }
}