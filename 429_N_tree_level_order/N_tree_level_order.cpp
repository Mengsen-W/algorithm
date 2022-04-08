/*
 * @Date: 2022-04-08 01:51:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-08 03:06:30
 * @FilePath: /algorithm/429_N_tree_level_order/N_tree_level_order.cpp
 */

#include <cassert>
#include <queue>
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
  vector<vector<int>> levelOrder(Node* root) {
    if (!root) {
      return {};
    }

    vector<vector<int>> ans;
    queue<Node*> q;
    q.push(root);

    while (!q.empty()) {
      int cnt = q.size();
      vector<int> level;
      for (int i = 0; i < cnt; ++i) {
        Node* cur = q.front();
        q.pop();
        level.push_back(cur->val);
        for (Node* child : cur->children) {
          q.push(child);
        }
      }
      ans.push_back(move(level));
    }

    return ans;
  }
};

int main() {
  assert((
      Solution().levelOrder(new Node{
          1, vector<Node*>{new Node{3, vector<Node*>{new Node{5}, new Node{6}}},
                           new Node{2}, new Node{4}}}) ==
      vector<vector<int>>{{1}, {3, 2, 4}, {5, 6}}));

  assert((
      Solution().levelOrder((new Node{
          1,
          vector<Node*>{
              new Node{2},
              new Node{
                  3,
                  vector<Node*>{
                      new Node{6},
                      new Node{7,
                               vector<Node*>{
                                   new Node{11, vector<Node*>{new Node{14}}},
                               }}}},
              new Node{4,
                       vector<Node*>{new Node{8, vector<Node*>{new Node{12}}}}},
              new Node{5,
                       vector<Node*>{new Node{9, vector<Node*>{new Node{13}}},
                                     new Node{10}}}}})) ==
      vector<vector<int>>{
          {1}, {2, 3, 4, 5}, {6, 7, 8, 9, 10}, {11, 12, 13}, {14}}));
}