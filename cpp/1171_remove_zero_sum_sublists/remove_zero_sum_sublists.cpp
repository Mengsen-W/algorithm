/*
 * @Date: 2023-06-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-11
 * @FilePath: /algorithm/cpp/1171_remove_zero_sum_sublists/remove_zero_sum_sublists.cpp
 */

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

#include <cassert>
#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
 public:
  ListNode* removeZeroSumSublists(ListNode* head) {
    ListNode* dummy = new ListNode(0);
    dummy->next = head;
    int prefix = 0;
    unordered_map<int, ListNode*> seen;
    for (ListNode* node = dummy; node; node = node->next) {
      prefix += node->val;
      seen[prefix] = node;
    }
    prefix = 0;
    for (ListNode* node = dummy; node; node = node->next) {
      prefix += node->val;
      node->next = seen[prefix]->next;
    }
    return dummy->next;
  }
};

string listNodeToString(ListNode* header) {
  string result = "[";
  if (header) {
    result += to_string(header->val);
    result += listNodeToString(header->next);
  }
  return result + "]";
}

int main() {
  {
    ListNode* header = new ListNode{1, new ListNode{2, new ListNode{-3, new ListNode{3, new ListNode{1}}}}};
    ListNode* ans = new ListNode{3, new ListNode{1}};
    assert(listNodeToString(Solution().removeZeroSumSublists(header)) == listNodeToString(ans));
  }

  {
    ListNode* header = new ListNode{1, new ListNode{2, new ListNode{3, new ListNode{-3, new ListNode{4}}}}};
    ListNode* ans = new ListNode{1, new ListNode{2, new ListNode{4}}};
    assert(listNodeToString(Solution().removeZeroSumSublists(header)) == listNodeToString(ans));
  }

  {
    ListNode* header = new ListNode{1, new ListNode{2, new ListNode{3, new ListNode{-3, new ListNode{-2}}}}};
    ListNode* ans = new ListNode{1};
    assert(listNodeToString(Solution().removeZeroSumSublists(header)) == listNodeToString(ans));
  }
}