/*
 * @Date: 2021-04-27 08:38:21
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-26
 */

#include <cassert>
#include <queue>
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

class Solution {
 public:
  int rangeSumBST(TreeNode *root, int low, int high) {
    int val = 0;
    std::queue<TreeNode *> q({root});
    while (!q.empty()) {
      TreeNode *node = q.front();
      q.pop();
      if (node == nullptr) continue;
      if (node->val > high) {
        q.push(node->left);
      } else if (node->val < low) {
        q.push(node->right);
      } else {
        val += node->val;
        q.push(node->left);
        q.push(node->right);
      }
    }
    return val;
  }
};

int main() {
  std::vector<std::tuple<TreeNode *, int, int, int>> tests{
      {
          new TreeNode{10, new TreeNode{5, new TreeNode{3, nullptr, nullptr}, new TreeNode{7, nullptr, nullptr}},
                       new TreeNode{15, nullptr, new TreeNode{18, nullptr, nullptr}}},
          7,
          15,
          32,
      },
      {
          new TreeNode{10,
                       new TreeNode{5, new TreeNode{3, new TreeNode{3, nullptr, nullptr}, nullptr},
                                    new TreeNode{7, new TreeNode{6, nullptr, nullptr}, nullptr}},
                       new TreeNode{15, new TreeNode{13, nullptr, nullptr}, new TreeNode{18, nullptr, nullptr}}},
          6,
          10,
          23,
      },
  };

  for (auto &[root, low, high, ans] : tests) {
    assert(Solution().rangeSumBST(root, low, high) == ans);
  }
}
