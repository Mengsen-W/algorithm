/*
 * @Date: 2023-12-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-15
 * @FilePath: /algorithm/cpp/2415_reverse_odd_levels/reverse_odd_levels.cpp
 */

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

#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  TreeNode *reverseOddLevels(TreeNode *root) {
    queue<TreeNode *> qu;
    qu.emplace(root);
    bool isOdd = false;
    while (!qu.empty()) {
      int sz = qu.size();
      vector<TreeNode *> arr;
      for (int i = 0; i < sz; i++) {
        TreeNode *node = qu.front();
        qu.pop();
        if (isOdd) {
          arr.emplace_back(node);
        }
        if (node->left) {
          qu.emplace(node->left);
          qu.emplace(node->right);
        }
      }
      if (isOdd) {
        for (int l = 0, r = sz - 1; l < r; l++, r--) {
          swap(arr[l]->val, arr[r]->val);
        }
      }
      isOdd ^= true;
    }
    return root;
  }
};

int main() {
  vector<tuple<TreeNode *, TreeNode *>> tests{
      {
          new TreeNode{2, new TreeNode{3, new TreeNode{8}, new TreeNode{13}},
                       new TreeNode{5, new TreeNode{21}, new TreeNode{34}}},
          new TreeNode{2, new TreeNode{5, new TreeNode{8}, new TreeNode{13}},
                       new TreeNode{3, new TreeNode{21}, new TreeNode{34}}},
      },
      {
          new TreeNode{7, new TreeNode{13}, new TreeNode{11}},
          new TreeNode{7, new TreeNode{11}, new TreeNode{13}},
      },
      {
          new TreeNode{0,
                       new TreeNode{1, new TreeNode{0, new TreeNode{1}, new TreeNode{1}},
                                    new TreeNode{0, new TreeNode{1}, new TreeNode{1}}},
                       new TreeNode{2, new TreeNode{0, new TreeNode{2}, new TreeNode{2}},
                                    new TreeNode{0, new TreeNode{2}, new TreeNode{2}}}},
          new TreeNode{0,
                       new TreeNode{2, new TreeNode{0, new TreeNode{2}, new TreeNode{2}},
                                    new TreeNode{0, new TreeNode{2}, new TreeNode{2}}},
                       new TreeNode{1, new TreeNode{0, new TreeNode{1}, new TreeNode{1}},
                                    new TreeNode{0, new TreeNode{1}, new TreeNode{1}}}},
      },
  };

  for (auto &[root, ans] : tests) {
    assert(isSameTree(Solution().reverseOddLevels(root), ans) == true);
  }
}
