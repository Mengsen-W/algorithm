/*
 * @Date: 2023-05-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-22
 * @FilePath: /algorithm/cpp/1080_sufficient_subset/sufficient_subset.cpp
 */

#include <cassert>
#include <cstddef>
#include <iostream>
#include <list>
#include <queue>
#include <string>
#include <vector>

using namespace std;

struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

class Solution {
 public:
  bool checkSufficientLeaf(TreeNode* node, int sum, int limit) {
    if (!node) {
      return false;
    }
    if (node->left == nullptr && node->right == nullptr) {
      return node->val + sum >= limit;
    }
    bool haveSufficientLeft = checkSufficientLeaf(node->left, sum + node->val, limit);
    bool haveSufficientRight = checkSufficientLeaf(node->right, sum + node->val, limit);
    if (!haveSufficientLeft) {
      node->left = nullptr;
    }
    if (!haveSufficientRight) {
      node->right = nullptr;
    }
    return haveSufficientLeft || haveSufficientRight;
  }

  TreeNode* sufficientSubset(TreeNode* root, int limit) {
    bool haveSufficient = checkSufficientLeaf(root, 0, limit);
    return haveSufficient ? root : nullptr;
  }
};

vector<vector<int>> levelOrder(TreeNode* root) {
  vector<vector<int>> ret;
  if (!root) {
    return ret;
  }

  queue<TreeNode*> q;
  q.push(root);
  while (!q.empty()) {
    int currentLevelSize = q.size();
    ret.push_back(vector<int>());
    for (int i = 1; i <= currentLevelSize; ++i) {
      auto node = q.front();
      q.pop();
      ret.back().push_back(node->val);
      if (node->left) q.push(node->left);
      if (node->right) q.push(node->right);
    }
  }

  return ret;
}

int main() {
  {
    TreeNode* root = new TreeNode{1,
                                  new TreeNode{2, new TreeNode{4, new TreeNode{8}, new TreeNode{9}},
                                               new TreeNode{-99, new TreeNode{-99}, new TreeNode{-99}}},
                                  new TreeNode{3, new TreeNode{-99, new TreeNode{12}, new TreeNode{13}},
                                               new TreeNode{7, new TreeNode{-99}, new TreeNode{14}}}};
    int limit = 1;
    TreeNode* ans = new TreeNode{1, new TreeNode{2, new TreeNode{4, new TreeNode{8}, new TreeNode{9}}, nullptr},
                                 new TreeNode{3, nullptr, new TreeNode{7, nullptr, new TreeNode{14}}}};
    assert(levelOrder(Solution().sufficientSubset(root, limit)) == levelOrder(ans));
  }

  {
    TreeNode* root = new TreeNode{5, new TreeNode{4, new TreeNode{11, new TreeNode{7}, new TreeNode{1}}, nullptr},
                                  new TreeNode{8, new TreeNode{17}, new TreeNode{4, new TreeNode{5}, new TreeNode{3}}}};
    int limit = 22;
    TreeNode* ans = new TreeNode{5, new TreeNode{4, new TreeNode{11, new TreeNode{7}, nullptr}, nullptr},
                                 new TreeNode{8, new TreeNode{17}, new TreeNode{4, new TreeNode{5}, nullptr}}};
    assert(levelOrder(Solution().sufficientSubset(root, limit)) == levelOrder(ans));
  }

  {
    TreeNode* root =
        new TreeNode{1, new TreeNode{2, new TreeNode{-5}, nullptr}, new TreeNode{-3, new TreeNode{4}, nullptr}};
    int limit = -1;
    TreeNode* ans = new TreeNode{1, nullptr, new TreeNode{-3, new TreeNode{4}, nullptr}};
    assert(levelOrder(Solution().sufficientSubset(root, limit)) == levelOrder(ans));
  }
}