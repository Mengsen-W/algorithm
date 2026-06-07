#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

// Definition for a binary tree node.
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
  TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
    unordered_map<int, bool> isRoot;      // 数值对应的节点是否为根节点的哈希表
    unordered_map<int, TreeNode*> nodes;  // 数值与对应节点的哈希表
    for (const auto& d : descriptions) {
      int p = d[0];
      int c = d[1];
      bool left = d[2];
      if (!isRoot.count(p)) {
        isRoot[p] = true;
      }
      isRoot[c] = false;
      // 创建或更新节点
      if (!nodes.count(p)) {
        nodes[p] = new TreeNode(p);
      }
      if (!nodes.count(c)) {
        nodes[c] = new TreeNode(c);
      }
      if (left) {
        nodes[p]->left = nodes[c];
      } else {
        nodes[p]->right = nodes[c];
      }
    }
    // 寻找根节点
    int root = -1;
    for (const auto& [val, r] : isRoot) {
      if (r) {
        root = val;
        break;
      }
    }
    return nodes[root];
  }
};

bool isSameTree(TreeNode* p, TreeNode* q) {
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
  vector<tuple<vector<vector<int>>, TreeNode*>> tests{
      {
          {{20, 15, 1}, {20, 17, 0}, {50, 20, 1}, {50, 80, 0}, {80, 19, 1}},
          new TreeNode(50, new TreeNode(20, new TreeNode(15), new TreeNode(17)),
                       new TreeNode(80, new TreeNode(19), nullptr)),
      },
      {
          {{1, 2, 1}, {2, 3, 0}, {3, 4, 1}},
          new TreeNode(1, new TreeNode(2, nullptr, new TreeNode(3, new TreeNode(4), nullptr)), nullptr),
      },
  };

  for (auto& [descriptions, expected] : tests) {
    assert(isSameTree(Solution().createBinaryTree(descriptions), expected));
  }
}