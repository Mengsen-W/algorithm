/*
 * @Date: 2022-03-18 23:45:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-19 00:01:34
 * @FilePath: /algorithm/606_tree2str/tree2str.cpp
 */

#include <cassert>
#include <stack>
#include <string>
#include <unordered_set>

using namespace std;

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
  string tree2str(TreeNode *root) {
    if (root == nullptr) {
      return "";
    }
    if (root->left == nullptr && root->right == nullptr) {
      return to_string(root->val);
    }
    if (root->right == nullptr) {
      return to_string(root->val) + "(" + tree2str(root->left) + ")";
    }
    return to_string(root->val) + "(" + tree2str(root->left) + ")(" +
           tree2str(root->right) + ")";
  }

  string tree2str1(TreeNode *root) {
    string ans = "";
    stack<TreeNode *> st;
    st.push(root);
    unordered_set<TreeNode *> vis;
    while (!st.empty()) {
      auto node = st.top();
      if (vis.count(node)) {
        if (node != root) {
          ans += ")";
        }
        st.pop();
      } else {
        vis.insert(node);
        if (node != root) {
          ans += "(";
        }
        ans += to_string(node->val);
        if (node->left == nullptr && node->right != nullptr) {
          ans += "()";
        }
        if (node->right != nullptr) {
          st.push(node->right);
        }
        if (node->left != nullptr) {
          st.push(node->left);
        }
      }
    }
    return ans;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode(1, new TreeNode(2, new TreeNode(4), nullptr),
                                  new TreeNode(3));
    assert(Solution().tree2str(root) == "1(2(4))(3)");
    assert(Solution().tree2str1(root) == "1(2(4))(3)");
  }
  {
    TreeNode *root = new TreeNode(1, new TreeNode(2, nullptr, new TreeNode(4)),
                                  new TreeNode(3));
    assert(Solution().tree2str(root) == "1(2()(4))(3)");
    assert(Solution().tree2str1(root) == "1(2()(4))(3)");
  }
}