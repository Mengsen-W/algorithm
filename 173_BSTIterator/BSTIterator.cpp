/*
 * @Date: 2021-03-28 10:19:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-28 10:29:44
 */

#include <cassert>
#include <stack>

using namespace std;

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class BSTIterator {
 private:
  TreeNode *cur;
  stack<TreeNode *> stk;

 public:
  BSTIterator(TreeNode *root) : cur(root) {}

  int next() {
    while (cur != nullptr) {
      stk.push(cur);
      cur = cur->left;
    }
    cur = stk.top();
    stk.pop();
    int ret = cur->val;
    cur = cur->right;
    return ret;
  }

  bool hasnext() { return cur != nullptr || !stk.empty(); }
};

int main(int argc, char **argv) {
  TreeNode *root = new TreeNode(
      7, new TreeNode(3), new TreeNode(15, new TreeNode(9), new TreeNode(20)));
  BSTIterator it{root};
  assert(it.next() == 3);
  assert(it.next() == 7);
  assert(it.hasnext());
  assert(it.next() == 9);
  assert(it.hasnext());
  assert(it.next() == 15);
  assert(it.hasnext());
  assert(it.next() == 20);
  assert(!it.hasnext());
}