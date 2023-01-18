/*
 * @Date: 2022-08-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-27
 * @FilePath: /algorithm/662_width_of_binary_tree/width_of_binary_tree.cpp
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
#include <vector>

using namespace std;

class Solution {
 public:
  int widthOfBinaryTree(TreeNode *root) {
    unsigned long long res = 1;
    vector<pair<TreeNode *, unsigned long long>> arr;
    arr.emplace_back(root, 1L);
    while (!arr.empty()) {
      vector<pair<TreeNode *, unsigned long long>> tmp;
      for (auto &[node, index] : arr) {
        if (node->left) {
          tmp.emplace_back(node->left, index * 2);
        }
        if (node->right) {
          tmp.emplace_back(node->right, index * 2 + 1);
        }
      }
      res = max(res, arr.back().second - arr[0].second + 1);
      arr = move(tmp);
    }
    return res;
  }
};

int main() {
  {
    TreeNode *root =
        new TreeNode{1, new TreeNode{3, new TreeNode{5}, new TreeNode{3}}, new TreeNode{2, nullptr, new TreeNode{9}}};
    int ans = 4;
    assert(Solution().widthOfBinaryTree(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{1, new TreeNode{3, new TreeNode{5, new TreeNode{6}, nullptr}, nullptr},
                                  new TreeNode{2, nullptr, new TreeNode{9, new TreeNode{7}, nullptr}}};
    int ans = 7;
    assert(Solution().widthOfBinaryTree(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{1, new TreeNode{3, new TreeNode{5}, nullptr}, new TreeNode{2, nullptr, nullptr}};
    int ans = 2;
    assert(Solution().widthOfBinaryTree(root) == ans);
  }
}