/*
 * @Date: 2024-02-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-22
 * @FilePath: /algorithm/cpp/889_construct_from_pre_post/construct_from_pre_post.cpp
 */

// Definition for a binary tree node.
#include <cassert>
#include <tuple>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <functional>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  TreeNode *constructFromPrePost(vector<int> &preorder, vector<int> &postorder) {
    int n = preorder.size();
    unordered_map<int, int> postMap;
    for (int i = 0; i < n; i++) {
      postMap[postorder[i]] = i;
    }
    function<TreeNode *(int, int, int, int)> dfs = [&](int preLeft, int preRight, int postLeft,
                                                       int postRight) -> TreeNode * {
      if (preLeft > preRight) {
        return nullptr;
      }
      int leftCount = 0;
      if (preLeft < preRight) {
        leftCount = postMap[preorder[preLeft + 1]] - postLeft + 1;
      }
      return new TreeNode(preorder[preLeft], dfs(preLeft + 1, preLeft + leftCount, postLeft, postLeft + leftCount - 1),
                          dfs(preLeft + leftCount + 1, preRight, postLeft + leftCount, postRight - 1));
    };
    return dfs(0, n - 1, 0, n - 1);
  }
};

bool isSameTree(TreeNode *p, TreeNode *q) {
  if (p == nullptr && q == nullptr) {
    return true;
  } else if (p == nullptr || q == nullptr) {
    return false;
  } else if (p->val != q->val) {
    return false;
  } else {
    return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
  }
}

int main() {
  vector<tuple<vector<int>, vector<int>, TreeNode *>> tests{
      {
          {1, 2, 4, 5, 3, 6, 7},
          {4, 5, 2, 6, 7, 3, 1},
          new TreeNode{1, new TreeNode{2, new TreeNode{4}, new TreeNode{5}},
                       new TreeNode{3, new TreeNode{6}, new TreeNode{7}}},
      },
      {
          {1},
          {1},
          new TreeNode{1},
      },
  };

  for (auto &[preorder, postorder, ans] : tests) {
    assert(isSameTree(Solution().constructFromPrePost(preorder, postorder), ans) == true);
  }
}