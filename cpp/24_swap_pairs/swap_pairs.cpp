/*
 * @Date: 2023-08-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-06
 * @FilePath: /algorithm/cpp/24_swap_pairs/swap_pairs.cpp
 */

// Definition for singly-linked list.
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* swapPairs(ListNode* head) {
    ListNode* dummyHead = new ListNode(0);
    dummyHead->next = head;
    ListNode* temp = dummyHead;
    while (temp->next != nullptr && temp->next->next != nullptr) {
      ListNode* node1 = temp->next;
      ListNode* node2 = temp->next->next;
      temp->next = node2;
      node1->next = node2->next;
      node2->next = node1;
      temp = node1;
    }
    return dummyHead->next;
  }
};

std::string listNodeToString(ListNode* header) {
  std::string result = "[";
  if (header) {
    result += std::to_string(header->val);
    result += listNodeToString(header->next);
  }
  return result + "]";
}

int main() {
  std::vector<std::tuple<ListNode*, ListNode*>> tests{
      {
          new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4)))),
          new ListNode(2, new ListNode(1, new ListNode(4, new ListNode(3)))),
      },
      {
          nullptr,
          nullptr,
      },
      {
          new ListNode(1),
          new ListNode(1),
      },
  };

  for (auto& [head, ans] : tests) {
    assert(listNodeToString(Solution().swapPairs(head)) == listNodeToString(ans));
  }
}