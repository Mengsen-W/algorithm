/*
 * @Date: 2022-08-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-17
 * @FilePath: /algorithm/1302_deepest_leaves_sum/deepest_leaves_sum.cpp
 */

#include <cassert>
#include <queue>

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
  int deepestLeavesSum(TreeNode *root) {
    int sum = 0;
    queue<TreeNode *> qu;
    qu.emplace(root);
    while (!qu.empty()) {
      sum = 0;
      int size = qu.size();
      for (int i = 0; i < size; i++) {
        TreeNode *node = qu.front();
        qu.pop();
        sum += node->val;
        if (node->left != nullptr) {
          qu.emplace(node->left);
        }
        if (node->right != nullptr) {
          qu.emplace(node->right);
        }
      }
    }
    return sum;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode{1, new TreeNode{2, new TreeNode{4, new TreeNode{7}, nullptr}, new TreeNode{5}},
                                  new TreeNode{3, nullptr, new TreeNode{6, nullptr, new TreeNode{8}}}};
    assert(Solution().deepestLeavesSum(root) == 15);
  }
  {
    TreeNode *root = new TreeNode{
        6,
        new TreeNode{7, new TreeNode{2, new TreeNode{9}, nullptr}, new TreeNode{7, new TreeNode{1}, new TreeNode{4}}},
        new TreeNode{8, new TreeNode{1}, new TreeNode{3, nullptr, new TreeNode{5}}}};
    assert(Solution().deepestLeavesSum(root) == 19);
  }
}