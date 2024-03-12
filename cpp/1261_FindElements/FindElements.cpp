/*
 * @Date: 2024-03-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-12
 * @FilePath: /algorithm/cpp/1261_FindElements/FindElements.cpp
 */

// Definition for a binary tree node.
#include <cassert>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class FindElements {
 private:
  TreeNode *root;

 public:
  FindElements(TreeNode *root) {
    this->root = root;
    dfs(root, 0);
  }

  void dfs(TreeNode *node, int val) {
    if (node == nullptr) {
      return;
    }
    node->val = val;
    dfs(node->left, val * 2 + 1);
    dfs(node->right, val * 2 + 2);
  }

  bool find(int target) {
    target++;
    int k = 30 - __builtin_clz(target);
    TreeNode *node = root;
    while (k >= 0 && node != nullptr) {
      if ((target & (1 << k)) == 0) {
        node = node->left;
      } else {
        node = node->right;
      }
      k--;
    }
    return node != nullptr;
  }
};

int main() {
  FindElements *findElements = new FindElements{new TreeNode{-1, nullptr, new TreeNode{-1}}};
  assert(findElements->find(1) == false);  // return False
  assert(findElements->find(2) == true);   // return True
}