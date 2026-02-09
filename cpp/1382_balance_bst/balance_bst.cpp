#include <cassert>
#include <tuple>
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
  vector<int> inorderSeq;

  void getInorder(TreeNode* o) {
    if (o->left) {
      getInorder(o->left);
    }
    inorderSeq.push_back(o->val);
    if (o->right) {
      getInorder(o->right);
    }
  }

  TreeNode* build(int l, int r) {
    int mid = (l + r) >> 1;
    TreeNode* o = new TreeNode(inorderSeq[mid]);
    if (l <= mid - 1) {
      o->left = build(l, mid - 1);
    }
    if (mid + 1 <= r) {
      o->right = build(mid + 1, r);
    }
    return o;
  }

  TreeNode* balanceBST(TreeNode* root) {
    getInorder(root);
    return build(0, inorderSeq.size() - 1);
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
  vector<tuple<TreeNode*, TreeNode*>> tests{
      {
          new TreeNode{1, nullptr, new TreeNode{2, nullptr, new TreeNode{3, nullptr, new TreeNode{4}}}},
          new TreeNode{2, new TreeNode{1}, new TreeNode{3, nullptr, new TreeNode{4}}},
      },
      {
          new TreeNode{2, new TreeNode{1}, new TreeNode{3}},
          new TreeNode{2, new TreeNode{1}, new TreeNode{3}},
      },
  };

  for (auto& [p, q] : tests) {
    assert(isSameTree(Solution().balanceBST(p), q));
  }
}