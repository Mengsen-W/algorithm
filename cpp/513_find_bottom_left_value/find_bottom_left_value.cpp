
/*
 * @Author: Mengsen Wang mengsen_wang@163.com
 * @Date: 2022-06-22
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-22
 * @FilePath: /algorithm/513_find_bottom_left_value/find_bottom_left_value.cpp
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

using namespace std;

class Solution {
 public:
  int findBottomLeftValue(TreeNode *root) {
    int ret;
    queue<TreeNode *> q;
    q.push(root);
    while (!q.empty()) {
      auto p = q.front();
      q.pop();
      if (p->right) {
        q.push(p->right);
      }
      if (p->left) {
        q.push(p->left);
      }
      ret = p->val;
    }
    return ret;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode(2, new TreeNode(1), new TreeNode(3));
    assert(Solution().findBottomLeftValue(root) == 1);
  }

  {
    TreeNode *root = new TreeNode(1, new TreeNode(2, new TreeNode(4), nullptr),
                                  new TreeNode(3, new TreeNode(5, new TreeNode(7), nullptr), new TreeNode(6)));
    assert(Solution().findBottomLeftValue(root) == 7);
  }
}