/*
 * @Date: 2023-07-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-31
 * @FilePath: /algorithm/cpp/143_reorder_list/reorder_list.cpp
 */

#include <cassert>
#include <iostream>
#include <tuple>
#include <vector>
using namespace std;

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
 public:
  void reorderList(ListNode *head) {
    if (head == nullptr) {
      return;
    }
    vector<ListNode *> vec;
    ListNode *node = head;
    while (node != nullptr) {
      vec.emplace_back(node);
      node = node->next;
    }
    int i = 0, j = vec.size() - 1;
    while (i < j) {
      vec[i]->next = vec[j];
      i++;
      if (i == j) {
        break;
      }
      vec[j]->next = vec[i];
      j--;
    }
    vec[i]->next = nullptr;
  }
};

vector<int> list_to_vec(ListNode *l) {
  ListNode *step = l;
  vector<int> ans{};
  while (step != nullptr) {
    ans.push_back(step->val);
    step = step->next;
  }
  return ans;
}

ListNode *vec_to_list(const vector<int> &nums) {
  int size = nums.size();
  if (size == 0) return nullptr;
  ListNode *res = new ListNode();
  ListNode *cur = res;
  for (int i = 0; i < size; ++i) {
    cur->val = nums[i];
    if (i == size - 1) {
      cur->next = nullptr;
    } else {
      cur->next = new ListNode();
    }
    cur = cur->next;
  }
  return res;
}

int main() {
  vector<tuple<ListNode *, vector<int>>> tests{
      {vec_to_list({1, 2, 3, 4}), vector<int>{1, 4, 2, 3}},
      {vec_to_list({1, 2, 3, 4, 5}), vector<int>{1, 5, 2, 4, 3}},
  };

  for (auto &[head, ans] : tests) {
    Solution().reorderList(head);
    assert(list_to_vec(head) == ans);
  }
}