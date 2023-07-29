/*
 * @Date: 2023-07-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-29
 * @FilePath: /algorithm/cpp/141_has_cycle/has_cycle.cpp
 */

// Definition for singly-linked list.
#include <cassert>
#include <iostream>
#include <tuple>
#include <vector>

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  bool hasCycle(ListNode* head) {
    if (head == nullptr || head->next == nullptr) {
      return false;
    }
    ListNode* slow = head;
    ListNode* fast = head->next;
    while (slow != fast) {
      if (fast == nullptr || fast->next == nullptr) {
        return false;
      }
      slow = slow->next;
      fast = fast->next->next;
    }
    return true;
  }
};

int main() {
  std::vector<std::tuple<ListNode*, bool>> tests{
      {new ListNode{1, nullptr}, false},
  };
  for (auto& [head, ans] : tests) {
    assert(Solution().hasCycle(head) == ans);
  }
}