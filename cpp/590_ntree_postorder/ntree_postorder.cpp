/*
 * @Date: 2022-03-12 02:01:09
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-19
 * @FilePath: /algorithm/cpp/590_ntree_postorder/ntree_postorder.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

// Definition for a Node.
class Node {
 public:
  int val;
  vector<Node *> children;

  Node() {}

  Node(int _val) { val = _val; }

  Node(int _val, vector<Node *> _children) {
    val = _val;
    children = _children;
  }
};

class Solution {
 public:
  vector<int> postorder(Node *root) {
    vector<int> res;
    if (root == nullptr) {
      return res;
    }

    stack<Node *> st;
    st.emplace(root);
    while (!st.empty()) {
      Node *node = st.top();
      st.pop();
      res.emplace_back(node->val);
      for (auto &item : node->children) {
        st.emplace(item);
      }
    }
    reverse(res.begin(), res.end());
    return res;
  }
};

int main() {
  {
    Node *root = new Node(1);
    root->children = {new Node(3), new Node(2), new Node(4)};
    root->children[0]->children = {new Node(5), new Node(6)};
    assert((Solution().postorder(root) == vector<int>{5, 6, 3, 2, 4, 1}));
  }
  {
    Node *root = new Node{1};
    vector<Node *> child{new Node{2}, new Node{3}, new Node{4}, new Node{5}};
    child[1]->children = {new Node{6}, new Node{7}};
    child[2]->children = {new Node{8}};
    child[3]->children = {new Node{9}, new Node{10}};
    child[1]->children[1]->children = {new Node{11}};
    child[1]->children[1]->children[0]->children = {new Node{14}};
    child[2]->children[0]->children = {new Node{12}};
    child[3]->children[0]->children = {new Node{13}};
    root->children = child;
    assert((Solution().postorder(root) == vector<int>{2, 6, 14, 11, 7, 3, 12, 8, 4, 13, 9, 10, 5, 1}));
  }
}