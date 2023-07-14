/*
 * @Date: 2023-07-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-14
 * @FilePath: /algorithm/cpp/979_distribute_coins/distribute_coins.cpp
 */

// Definition for a binary tree node.
#include <cassert>
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

#include <functional>

using namespace std;

class Solution {
 public:
  int distributeCoins(TreeNode *root) {
    int move = 0;

    function<int(const TreeNode *)> dfs = [&](const TreeNode *root) -> int {
      int moveleft = 0;
      int moveright = 0;
      if (root == nullptr) {
        return 0;
      }
      if (root->left) {
        moveleft = dfs(root->left);
      }
      if (root->right) {
        moveright = dfs(root->right);
      }
      move += abs(moveleft) + abs(moveright);
      return moveleft + moveright + root->val - 1;
    };

    dfs(root);
    return move;
  }
};

int main() {
  vector<tuple<TreeNode *, int>> test_cases{
      {new TreeNode{3, new TreeNode{0}, new TreeNode{0}}, 2},
      {new TreeNode{0, new TreeNode{3}, new TreeNode{0}}, 3},
      {new TreeNode{1, new TreeNode{0}, new TreeNode{2}}, 2},
      {new TreeNode{1, new TreeNode{0, nullptr, new TreeNode{3}}, new TreeNode{0}}, 4},
  };

  for (auto &[root, ans] : test_cases) {
    assert(Solution().distributeCoins(root) == ans);
  }
}