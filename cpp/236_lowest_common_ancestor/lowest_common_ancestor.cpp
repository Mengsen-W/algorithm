/*
 * @Date: 2024-02-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-09
 * @FilePath: /algorithm/cpp/236_lowest_common_ancestor/lowest_common_ancestor.cpp
 */

// Definition for a binary tree node.
#include <tuple>
#include <vector>

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
    if (root == nullptr || root == p || root == q) return root;
    TreeNode* left = lowestCommonAncestor(root->left, p, q);
    TreeNode* right = lowestCommonAncestor(root->right, p, q);
    if (left == nullptr && right == nullptr) return nullptr;  // 1.
    if (left == nullptr) return right;                        // 3.
    if (right == nullptr) return left;                        // 4.
    return root;                                              // 2. if(left != null and right != null)
  }
};

int main() {
  std::vector<std::tuple<TreeNode*, TreeNode*, TreeNode*, TreeNode*>> tests{
      {
          new TreeNode{3, new TreeNode{5, new TreeNode{6}, new TreeNode{2, new TreeNode{7}, new TreeNode{4}}},
                       new TreeNode{1, new TreeNode{0}, new TreeNode{8}}},
          new TreeNode{5, new TreeNode{6}, new TreeNode{2, new TreeNode{7}, new TreeNode{4}}},
          new TreeNode{1, new TreeNode{0}, new TreeNode{8}},
          new TreeNode{3, new TreeNode{5, new TreeNode{6}, new TreeNode{2, new TreeNode{7}, new TreeNode{4}}},
                       new TreeNode{1, new TreeNode{0}, new TreeNode{8}}},
      },
      {},
      {},
  };

  for (auto& [root, p, q, ans] : tests) {
  }
}