/*
 * @Date: 2024-02-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-16
 * @FilePath: /algorithm/cpp/103_zigzag_level_order/zigzag_level_order.cpp
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

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> zigzagLevelOrder(TreeNode *root) {
    vector<vector<int>> ans;
    if (!root) {
      return ans;
    }

    queue<TreeNode *> nodeQueue;
    nodeQueue.push(root);
    bool isOrderLeft = true;

    while (!nodeQueue.empty()) {
      deque<int> levelList;
      int size = nodeQueue.size();
      for (int i = 0; i < size; ++i) {
        auto node = nodeQueue.front();
        nodeQueue.pop();
        if (isOrderLeft) {
          levelList.push_back(node->val);
        } else {
          levelList.push_front(node->val);
        }
        if (node->left) {
          nodeQueue.push(node->left);
        }
        if (node->right) {
          nodeQueue.push(node->right);
        }
      }
      ans.emplace_back(vector<int>{levelList.begin(), levelList.end()});
      isOrderLeft = !isOrderLeft;
    }

    return ans;
  }
};

int main() {
  vector<tuple<TreeNode *, vector<vector<int>>>> tests{
      {
          new TreeNode{3, new TreeNode{9}, new TreeNode{20, new TreeNode{15}, new TreeNode{7}}},
          {{3}, {20, 9}, {15, 7}},
      },
      {
          new TreeNode{1},
          {{1}},
      },
      {
          nullptr,
          {},
      }};

  for (auto &[root, ans] : tests) {
    assert(Solution().zigzagLevelOrder(root) == ans);
  }
}