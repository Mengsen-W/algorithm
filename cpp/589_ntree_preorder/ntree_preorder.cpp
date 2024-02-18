/*
 * @Date: 2022-03-10 02:37:37
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-18
 * @FilePath: /algorithm/cpp/589_ntree_preorder/ntree_preorder.cpp
 */

#include <cassert>
#include <stack>
#include <vector>

using namespace std;

// Definition for a Node.
class Node {
 public:
  int val;
  vector<Node*> children;
  Node() {}
  Node(int _val) { val = _val; }
  Node(int _val, vector<Node*> _children) {
    val = _val;
    children = _children;
  }
};

class Solution {
 public:
  vector<int> preorder(Node* root) {
    vector<int> res;
    if (root == nullptr) {
      return res;
    }

    stack<Node*> st;
    st.emplace(root);
    while (!st.empty()) {
      Node* node = st.top();
      st.pop();
      res.emplace_back(node->val);
      for (auto it = node->children.rbegin(); it != node->children.rend(); it++) {
        st.emplace(*it);
      }
    }
    return res;
  }
};

int main() {
  {
    Node* root = new Node{1};
    vector<Node*> child{new Node{3}, new Node{2}, new Node{4}};
    child[0]->children = {new Node{5}, new Node{6}};
    root->children = child;
    assert((Solution().preorder(root) == vector<int>{1, 3, 5, 6, 2, 4}));
  }

  {
    Node* root = new Node{1};
    vector<Node*> child{new Node{2}, new Node{3}, new Node{4}, new Node{5}};
    child[1]->children = {new Node{6}, new Node{7}};
    child[2]->children = {new Node{8}};
    child[3]->children = {new Node{9}, new Node{10}};
    child[1]->children[1]->children = {new Node{11}};
    child[1]->children[1]->children[0]->children = {new Node{14}};
    child[2]->children[0]->children = {new Node{12}};
    child[3]->children[0]->children = {new Node{13}};
    root->children = child;
    assert((Solution().preorder(root) == vector<int>{1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10}));
  }
}
