/*
 * @Date: 2022-08-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-22
 * @FilePath: /algorithm/655_print_tree/print_tree.cpp
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
#include <queue>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int calDepth(TreeNode *root) {
    int res = -1;
    queue<TreeNode *> q;
    q.push(root);
    while (!q.empty()) {
      int len = q.size();
      res++;
      while (len) {
        len--;
        auto t = q.front();
        q.pop();
        if (t->left) {
          q.push(t->left);
        }
        if (t->right) {
          q.push(t->right);
        }
      }
    }
    return res;
  }

  vector<vector<string>> printTree(TreeNode *root) {
    int height = calDepth(root);
    int m = height + 1;
    int n = (1 << (height + 1)) - 1;
    vector<vector<string>> res(m, vector<string>(n, ""));
    queue<tuple<TreeNode *, int, int>> q;
    q.push({root, 0, (n - 1) / 2});
    while (!q.empty()) {
      auto t = q.front();
      q.pop();
      int r = get<1>(t), c = get<2>(t);
      res[r][c] = to_string(get<0>(t)->val);
      if (get<0>(t)->left) {
        q.push({get<0>(t)->left, r + 1, c - (1 << (height - r - 1))});
      }
      if (get<0>(t)->right) {
        q.push({get<0>(t)->right, r + 1, c + (1 << (height - r - 1))});
      }
    }
    return res;
  }
};

int main() {
  {
    TreeNode *root = new TreeNode{1, new TreeNode{2}, nullptr};
    vector<vector<string>> ans{{"", "1", ""}, {"2", "", ""}};
    assert(Solution().printTree(root) == ans);
  }

  {
    TreeNode *root = new TreeNode{1, new TreeNode{2, nullptr, new TreeNode{4}}, new TreeNode{3}};
    vector<vector<string>> ans{
        {"", "", "", "1", "", "", ""}, {"", "2", "", "", "", "3", ""}, {"", "", "4", "", "", "", ""}};
    assert(Solution().printTree(root) == ans);
  }
}