/*
 * @Date: 2023-02-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-03
 * @FilePath: /algorithm/cpp/1145_btree_game_winning_move/btree_game_winning_move.cpp
 */

#include <cassert>

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
  bool btreeGameWinningMove(TreeNode* root, int n, int x) {
    TreeNode* xNode = find(root, x);
    int leftSize = getSubtreeSize(xNode->left);
    if (leftSize >= (n + 1) / 2) {
      return true;
    }
    int rightSize = getSubtreeSize(xNode->right);
    if (rightSize >= (n + 1) / 2) {
      return true;
    }
    int remain = n - 1 - leftSize - rightSize;
    return remain >= (n + 1) / 2;
  }

  TreeNode* find(TreeNode* node, int x) {
    if (node == nullptr) {
      return nullptr;
    }
    if (node->val == x) {
      return node;
    }
    TreeNode* res = find(node->left, x);
    if (res != nullptr) {
      return res;
    } else {
      return find(node->right, x);
    }
  }

  int getSubtreeSize(TreeNode* node) {
    if (node == nullptr) {
      return 0;
    }
    return 1 + getSubtreeSize(node->left) + getSubtreeSize(node->right);
  }
};

int main() {
  {
    TreeNode* root = new TreeNode{1,
                                  new TreeNode{2, new TreeNode{4, new TreeNode{8}, new TreeNode{9}},
                                               new TreeNode{5, new TreeNode{10}, new TreeNode{11}}},
                                  new TreeNode{3, new TreeNode{6}, new TreeNode{7}}};
    int n = 11;
    int x = 3;
    bool ans = true;
    assert(Solution().btreeGameWinningMove(root, n, x) == ans);
  }

  {
    TreeNode* root = new TreeNode{1, new TreeNode{2}, new TreeNode{3}};
    int n = 3;
    int x = 1;
    bool ans = false;
    assert(Solution().btreeGameWinningMove(root, n, x) == ans);
  }
}