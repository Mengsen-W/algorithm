/*
 * @Date: 2022-09-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-05
 * @FilePath: /algorithm/652_find_duplicate_subtrees/find_duplicate_subtrees.cpp
 */

// Definition for a binary tree node.
struct TreeNode {
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode* left, TreeNode* right) : val(x), left(left), right(right) {}
};

#include <cassert>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<TreeNode*> findDuplicateSubtrees(TreeNode* root) {
    dfs(root);
    return {repeat.begin(), repeat.end()};
  }

  int dfs(TreeNode* node) {
    if (!node) {
      return 0;
    }
    auto tri = tuple{node->val, dfs(node->left), dfs(node->right)};
    if (auto it = seen.find(tri); it != seen.end()) {
      repeat.insert(it->second.first);
      return it->second.second;
    } else {
      seen[tri] = {node, ++idx};
      return idx;
    }
  }

 private:
  static constexpr auto tri_hash = [fn = hash<int>()](const tuple<int, int, int>& o) -> size_t {
    auto&& [x, y, z] = o;
    return (fn(x) << 24) ^ (fn(y) << 8) ^ fn(z);
  };

  unordered_map<tuple<int, int, int>, pair<TreeNode*, int>, decltype(tri_hash)> seen{0, tri_hash};
  unordered_set<TreeNode*> repeat;
  int idx = 0;
};

bool isSameTree(TreeNode* p, TreeNode* q) {
  if (p == nullptr || q == nullptr) {
    return p == q;
  }

  if (p->val != q->val) {
    return false;
  }

  return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
}

int main() {
  {
    TreeNode* root = new TreeNode{1, new TreeNode{2, new TreeNode{4}, nullptr},
                                  new TreeNode{3, new TreeNode{2, new TreeNode{4}, nullptr}, new TreeNode{4}}};
    vector<TreeNode*> ans{new TreeNode{2, new TreeNode{4}, nullptr}, new TreeNode{4}};
    vector<TreeNode*> output{Solution().findDuplicateSubtrees(root)};
    for (int i = 0; i < ans.size(); ++i) assert(isSameTree(ans[i], output[i]));
  }

  {
    TreeNode* root = new TreeNode{2, new TreeNode{1}, new TreeNode{1}};
    vector<TreeNode*> ans{new TreeNode{1}};
    vector<TreeNode*> output{Solution().findDuplicateSubtrees(root)};
    for (int i = 0; i < ans.size(); ++i) assert(isSameTree(ans[i], output[i]));
  }

  {
    TreeNode* root =
        new TreeNode{2, new TreeNode{2, new TreeNode{3}, nullptr}, new TreeNode{2, new TreeNode{3}, nullptr}};
    vector<TreeNode*> ans{new TreeNode{2, new TreeNode{3}, nullptr}, new TreeNode{3}};
    vector<TreeNode*> output{Solution().findDuplicateSubtrees(root)};
    for (int i = 0; i < ans.size(); ++i) assert(isSameTree(ans[i], output[i]));
  }
}