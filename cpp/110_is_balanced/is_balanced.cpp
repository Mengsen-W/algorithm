#include <cassert>
#include <cmath>
#include <tuple>
#include <vector>

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
  int height(TreeNode* root) {
    if (root == nullptr) {
      return 0;
    }
    int leftHeight = height(root->left);
    int rightHeight = height(root->right);
    if (leftHeight == -1 || rightHeight == -1 || abs(leftHeight - rightHeight) > 1) {
      return -1;
    } else {
      return fmax(leftHeight, rightHeight) + 1;
    }
  }

  bool isBalanced(TreeNode* root) { return height(root) >= 0; }
};

int main() {
  std::vector<std::tuple<TreeNode*, bool>> tests{
      {
          new TreeNode(3, new TreeNode(9), new TreeNode(20, new TreeNode(15), new TreeNode(7))),
          true,
      },
      {
          new TreeNode(1, new TreeNode(2, new TreeNode(3, new TreeNode(4), new TreeNode(4)), new TreeNode(3)),
                       new TreeNode(2)),
          false,
      },
      {
          nullptr,
          true,
      },
  };

  for (auto& [root, ans] : tests) {
    assert(Solution().isBalanced(root) == ans);
  }
}