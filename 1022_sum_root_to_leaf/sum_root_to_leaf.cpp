/*
 * @Date: 2022-05-31 09:44:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-31 09:53:49
 * @FilePath: /algorithm/1022_sum_root_to_leaf/sum_root_to_leaf.cpp
 */

#include <cassert>
#include <stack>

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
  int sumRootToLeaf(TreeNode *root) {
    stack<TreeNode *> st;
    int val = 0, ret = 0;
    TreeNode *prev = nullptr;
    while (root != nullptr || !st.empty()) {
      while (root != nullptr) {
        val = (val << 1) | root->val;
        st.push(root);
        root = root->left;
      }
      root = st.top();
      if (root->right == nullptr || root->right == prev) {
        if (root->left == nullptr && root->right == nullptr) {
          ret += val;
        }
        val >>= 1;
        st.pop();
        prev = root;
        root = nullptr;
      } else {
        root = root->right;
      }
    }
    return ret;
  }
};

int main() {
  assert(Solution().sumRootToLeaf(new TreeNode(1, new TreeNode(0, new TreeNode(0), new TreeNode(1)),
                                               new TreeNode(1, new TreeNode(0), new TreeNode(1)))) == 22);
  assert(Solution().sumRootToLeaf(new TreeNode(0)) == 0);
}