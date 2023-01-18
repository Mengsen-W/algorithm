/*
 * @Date: 2021-12-25 01:39:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-25 06:38:43
 */

#include <cassert>
#include <climits>
#include <queue>

using namespace std;

//  Definition for a binary tree node.
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
  bool isEvenOddTree(TreeNode *root) {
    queue<TreeNode *> qu;
    qu.push(root);
    int level = 0;
    while (!qu.empty()) {
      int size = qu.size();
      int prev = level % 2 == 0 ? INT_MIN : INT_MAX;
      for (int i = 0; i < size; i++) {
        TreeNode *node = qu.front();
        qu.pop();
        int value = node->val;
        if (level % 2 == value % 2 || (level % 2 == 0 && value <= prev) ||
            (level % 2 == 1 && value >= prev)) {
          return false;
        }

        prev = value;
        if (node->left != nullptr) {
          qu.push(node->left);
        }
        if (node->right != nullptr) {
          qu.push(node->right);
        }
      }
      level++;
    }
    return true;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode(
        1,
        new TreeNode(10, new TreeNode(3, new TreeNode(12), new TreeNode(8)),
                     nullptr),
        new TreeNode(4, new TreeNode(7, new TreeNode(6), nullptr),
                     new TreeNode(9, nullptr, new TreeNode(2))));
    assert(Solution().isEvenOddTree(root) == true);
  }
  {
    TreeNode *root =
        new TreeNode(5, new TreeNode(4, new TreeNode(3), new TreeNode(3)),
                     new TreeNode(2, new TreeNode(7), nullptr));
    assert(Solution().isEvenOddTree(root) == false);
  }
  {
    TreeNode *root =
        new TreeNode(5, new TreeNode(9, new TreeNode(3), new TreeNode(5)),
                     new TreeNode(1, new TreeNode(7), nullptr));
    assert(Solution().isEvenOddTree(root) == false);
  }
  {
    TreeNode *root = new TreeNode(1);
    assert(Solution().isEvenOddTree(root) == true);
  }
  {
    TreeNode *root = new TreeNode(
        11,
        new TreeNode(
            8,
            new TreeNode(1, new TreeNode(30, new TreeNode(17), nullptr),
                         new TreeNode(20)),
            new TreeNode(3, new TreeNode(18), new TreeNode(16))),
        new TreeNode(6, new TreeNode(9, new TreeNode(12), new TreeNode(10)),
                     new TreeNode(11, new TreeNode(4), new TreeNode(2))));
    assert(Solution().isEvenOddTree(root) == true);
  }
}