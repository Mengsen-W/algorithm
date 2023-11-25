/*
 * @Date: 2023-11-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-25
 * @FilePath: /algorithm/cpp/1457_pseudo_palindromic_paths/pseudo_palindromic_paths.cpp
 */

#include <cassert>
#include <tuple>
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
 public:
  int pseudoPalindromicPaths(TreeNode *root) {
    vector<int> counter(10);
    return dfs(root, counter);
  }

  int dfs(TreeNode *root, vector<int> &counter) {
    if (root == nullptr) {
      return 0;
    }
    counter[root->val]++;
    int res = 0;
    if (root->left == nullptr && root->right == nullptr) {
      if (isPseudoPalindrome(counter)) {
        res = 1;
      }
    } else {
      res = dfs(root->left, counter) + dfs(root->right, counter);
    }
    counter[root->val]--;
    return res;
  }

  bool isPseudoPalindrome(const vector<int> &counter) {
    int odd = 0;
    for (int value : counter) {
      if (value % 2 == 1) {
        odd++;
      }
    }
    return odd <= 1;
  }
};

int main() {
  vector<tuple<TreeNode *, int>> tests{
      {
          new TreeNode{2, new TreeNode{3, new TreeNode{3}, new TreeNode{1}}, new TreeNode{1, nullptr, new TreeNode{1}}},
          2,
      },
      {
          new TreeNode{2, new TreeNode{1, new TreeNode{1}, new TreeNode{3, nullptr, new TreeNode{1}}}, new TreeNode{1}},
          1,
      },
      {
          new TreeNode{1},
          1,
      },
  };

  for (auto &[root, ans] : tests) {
    assert(Solution().pseudoPalindromicPaths(root) == ans);
  }
}