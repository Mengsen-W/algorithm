/*
 * @Date: 2022-05-01 09:27:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-01 10:07:59
 * @FilePath: /algorithm/1305_get_all_elements_in_two_bst/get_all_elements.cpp
 */

#include <cassert>
#include <vector>

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
  void inorder(TreeNode *node, vector<int> &res) {
    if (node) {
      inorder(node->left, res);
      res.push_back(node->val);
      inorder(node->right, res);
    }
  }

 public:
  vector<int> getAllElements(TreeNode *root1, TreeNode *root2) {
    vector<int> nums1, nums2;
    inorder(root1, nums1);
    inorder(root2, nums2);

    vector<int> merged;
    auto p1 = nums1.begin(), p2 = nums2.begin();
    while (true) {
      if (p1 == nums1.end()) {
        merged.insert(merged.end(), p2, nums2.end());
        break;
      }
      if (p2 == nums2.end()) {
        merged.insert(merged.end(), p1, nums1.end());
        break;
      }
      if (*p1 < *p2) {
        merged.push_back(*p1++);
      } else {
        merged.push_back(*p2++);
      }
    }
    return merged;
  }
};

int main() {
  {
    TreeNode *root1 = new TreeNode(2, new TreeNode(1), new TreeNode(4));
    TreeNode *root2 = new TreeNode(1, new TreeNode(0), new TreeNode(3));
    assert((Solution().getAllElements(root1, root2) == vector<int>{0, 1, 1, 2, 3, 4}));
  }

  {
    TreeNode *root1 = new TreeNode(1, nullptr, new TreeNode(8));
    TreeNode *root2 = new TreeNode(1, new TreeNode(8), nullptr);
    assert((Solution().getAllElements(root1, root2) == vector<int>{1, 8, 1, 8}));
  }
}