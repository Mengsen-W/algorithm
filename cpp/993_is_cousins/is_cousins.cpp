/*
 * @Date: 2021-05-17 08:36:11
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-08
 */

#include <cassert>
#include <queue>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

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
  using PTT = pair<TreeNode *, TreeNode *>;
  bool isCousins(TreeNode *root, int x, int y) {
    // 使用队列q来进行bfs
    // 其中pair中，p.first 记录当前结点的指针，p.second
    // 记录当前结点的父结点的指针
    queue<PTT> q;
    q.push({root, nullptr});
    while (!q.empty()) {
      int n = q.size();
      vector<TreeNode *> rec_parent;
      for (int i = 0; i < n; i++) {
        auto [cur, parent] = q.front();
        q.pop();
        if (cur->val == x || cur->val == y) rec_parent.push_back(parent);
        if (cur->left) q.push({cur->left, cur});
        if (cur->right) q.push({cur->right, cur});
      }
      // `x` 和 `y` 都没出现
      if (rec_parent.size() == 0) continue;
      // `x` 和 `y` 只出现一个
      else if (rec_parent.size() == 1)
        return false;
      // `x` 和 `y` 都出现了
      else if (rec_parent.size() == 2)
        // `x` 和 `y` 父节点 相同/不相同 ？
        return rec_parent[0] != rec_parent[1];
    }
    return false;
  }
};

class Solution1 {
 public:
  bool isCousins(TreeNode *root, int x, int y) {
    assert(root && "Unexpected empty tree!");
    dfs(root, x, y, 0);
    return (child2Parent[x] != child2Parent[y] && node2level[x] == node2level[y]);
  }

 private:
  unordered_map<int, int> child2Parent;
  unordered_map<int, int> node2level;

  void dfs(TreeNode *root, int x, int y, int level) {
    if (!root) {
      return;
    }

    if (root->left) {
      child2Parent[root->left->val] = root->val;
      node2level[root->left->val] = level + 1;
    }

    if (root->right) {
      child2Parent[root->right->val] = root->val;
      node2level[root->right->val] = level + 1;
    }

    dfs(root->left, x, y, level + 1);
    dfs(root->right, x, y, level + 1);
  }
};

int main() {
  vector<tuple<TreeNode *, int, int, bool>> tests{
      {new TreeNode{1, new TreeNode{2, new TreeNode{4, nullptr, nullptr}, nullptr}, new TreeNode{3, nullptr, nullptr}},
       4, 3, false},
      {new TreeNode{1, new TreeNode{2, nullptr, new TreeNode{4, nullptr, nullptr}},
                    new TreeNode{3, nullptr, new TreeNode{5, nullptr, nullptr}}},
       5, 4, true},
      {new TreeNode{1, new TreeNode{2, nullptr, new TreeNode{4, nullptr, nullptr}}, new TreeNode{3, nullptr, nullptr}},
       2, 3, false},
  };
  for (auto &[root, x, y, ans] : tests) {
    assert(Solution().isCousins(root, x, y) == ans);
  }
  {
    TreeNode *root =
        new TreeNode{1, new TreeNode{2, new TreeNode{4, nullptr, nullptr}, nullptr}, new TreeNode{3, nullptr, nullptr}};
    assert(!Solution{}.isCousins(root, 4, 3));
    assert(!Solution1{}.isCousins(root, 4, 3));
  }
  {
    TreeNode *root = new TreeNode{1, new TreeNode{2, nullptr, new TreeNode{4, nullptr, nullptr}},
                                  new TreeNode{3, nullptr, new TreeNode{5, nullptr, nullptr}}};
    assert(Solution{}.isCousins(root, 5, 4));
    assert(Solution1{}.isCousins(root, 5, 4));
  }
  {
    TreeNode *root =
        new TreeNode{1, new TreeNode{2, nullptr, new TreeNode{4, nullptr, nullptr}}, new TreeNode{3, nullptr, nullptr}};
    assert(!Solution{}.isCousins(root, 2, 3));
    assert(!Solution1{}.isCousins(root, 2, 3));
  }
}
