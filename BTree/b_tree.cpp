/*
 * @Date: 2021-09-04 13:13:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-04 14:10:13
 * @FilePath: /algorithm/BTree/b_tree.cpp
 * @Description: file content
 */

#include <cassert>
#include <iostream>
#include <queue>

template <int T>
class node {
 public:
  bool insert(int n) {
    // 递归调用
    if (node_and_child.size() < T) {
      // 不满
      for (int i = 0; i < T - 1; ++i) {
        if (node_and_child[i].first <= data &&
            node_and_child[i + 1].first > data) {
          node_and_child[i + 1].insert(
              std::pair<int, std::vector<node*>>{n, std::vector(T, nullptr)});
        } else {
          node_and_child.push_back(
              std::pair<int, std::vector<node*>>{n, std::vector(T, nullptr)})
        }
      }
    } else {
      // 已满
      for (auto child : node_and_child) {
        // 找每一个节点
        
      }
    }
  }

 private:
  std::vector<std::pair<int, std::vector<node*>>> node_and_child;
};

template <int T>
class M_Tree {
 public:
  M_Tree() : size(T), root() {}
  ~M_Tree() {}
  M_Tree* root() const { return root; }
  M_Tree* delete_child(int n) {}
  bool insert(int n) {
    if (root.size() < size) {
      root.push_back(n);
    } else {
      // 调用节点insert
    }
  }
  bool search() {}

 private:
  int size;
  node* root;
};

int main() { M_Tree<5>{}; }