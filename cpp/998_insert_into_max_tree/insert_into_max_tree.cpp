/*
 * @Date: 2022-08-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-30
 * @FilePath: /algorithm/998_insert_into_max_tree/insert_into_max_tree.cpp
 */

#include <cassert>

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  TreeNode* insertIntoMaxTree(TreeNode* root, int val) {
    TreeNode* parent = nullptr;
    TreeNode* cur = root;
    while (cur) {
      if (val > cur->val) {
        if (!parent) {
          return new TreeNode(val, root, nullptr);
        }
        TreeNode* node = new TreeNode(val, cur, nullptr);
        parent->right = node;
        return root;
      } else {
        parent = cur;
        cur = cur->right;
      }
    }
    parent->right = new TreeNode(val);
    return root;
  }
};

bool isSameTree(TreeNode* p, TreeNode* q) {
  if (p == nullptr || q == nullptr) return p == q;
  if (p->val != q->val) return false;
  return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
}

int main() {
  {
    TreeNode* root = new TreeNode{4, new TreeNode{1}, new TreeNode{3, new TreeNode{2}, nullptr}};
    int val = 5;
    TreeNode* ans =
        new TreeNode{5, new TreeNode{4, new TreeNode{1}, new TreeNode{3, new TreeNode{2}, nullptr}}, nullptr};
    assert(isSameTree(Solution().insertIntoMaxTree(root, val), ans));
  }

  {
    TreeNode* root = new TreeNode{5, new TreeNode{2, nullptr, new TreeNode{1}}, new TreeNode{4}};
    int val = 3;
    TreeNode* ans =
        new TreeNode{5, new TreeNode{2, nullptr, new TreeNode{1}}, new TreeNode{4, nullptr, new TreeNode{3}}};
    assert(isSameTree(Solution().insertIntoMaxTree(root, val), ans));
  }

  {
    TreeNode* root = new TreeNode{5, new TreeNode{2, nullptr, new TreeNode{1}}, new TreeNode{3}};
    int val = 4;
    TreeNode* ans =
        new TreeNode{5, new TreeNode{2, nullptr, new TreeNode{1}}, new TreeNode{4, new TreeNode{3}, nullptr}};
    assert(isSameTree(Solution().insertIntoMaxTree(root, val), ans));
  }
}