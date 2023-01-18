/*
 * @Date: 2021-07-27 17:37:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-27 18:44:00
 */

#include <cassert>
#include <set>

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right)
      : val(x), left(left), right(right) {}
};

class Solution {
 public:
  std::set<int> set;
  int findSecondMinimumValue(TreeNode* root) {
    if (root == nullptr) return -1;
    dfs(root);
    if (set.size() < 2) return -1;
    return *(++set.begin());
  }
  void dfs(TreeNode* root) {
    if (root == nullptr) return;
    set.insert(root->val);
    dfs(root->left);
    dfs(root->right);
    return;
  }
};

int main() {
  {
    TreeNode* root =
        new TreeNode{2, new TreeNode{2, nullptr, nullptr},
                     new TreeNode{5, new TreeNode{5, nullptr, nullptr},
                                  new TreeNode{7, nullptr, nullptr}}};
    assert(Solution{}.findSecondMinimumValue(root) == 5);
  }
  {
    TreeNode* root = new TreeNode{2, new TreeNode{2, nullptr, nullptr},
                                  new TreeNode{2, nullptr, nullptr}};
    assert(Solution{}.findSecondMinimumValue(root) == -1);
  }
}