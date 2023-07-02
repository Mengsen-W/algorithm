/*
 * @Date: 2023-07-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-02
 * @FilePath: /algorithm/cpp/2_add_two_numbers/add_two_numbers.cpp
 */

//  Definition for singly-linked list.
#include <cassert>
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
  ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
    ListNode *head = nullptr, *tail = nullptr;
    int carry = 0;
    while (l1 || l2) {
      int n1 = l1 ? l1->val : 0;
      int n2 = l2 ? l2->val : 0;
      int sum = n1 + n2 + carry;
      if (!head) {
        head = tail = new ListNode(sum % 10);
      } else {
        tail->next = new ListNode(sum % 10);
        tail = tail->next;
      }
      carry = sum / 10;
      if (l1) {
        l1 = l1->next;
      }
      if (l2) {
        l2 = l2->next;
      }
    }
    if (carry > 0) {
      tail->next = new ListNode(carry);
    }
    return head;
  }
};

bool is_same(ListNode* l1, ListNode* l2) {
  if (l1 == nullptr && l2 == nullptr) {
    return true;
  }
  if (l1 != nullptr && l2 != nullptr) {
    return l1->val == l2->val && is_same(l1->next, l2->next);
  }
  return false;
}

int main() {
  std::vector<std::tuple<ListNode*, ListNode*, ListNode*>> test_cases{
      {
          new ListNode(2, new ListNode(4, new ListNode(3))),
          new ListNode(5, new ListNode(6, new ListNode(4))),
          new ListNode(7, new ListNode(0, new ListNode(8))),
      },
      {
          new ListNode(0),
          new ListNode(0),
          new ListNode(0),
      },
      {
          new ListNode(
              9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9))))))),
          new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9)))),
          new ListNode(
              8, new ListNode(
                     9, new ListNode(
                            9, new ListNode(9, new ListNode(0, new ListNode(0, new ListNode(0, new ListNode(1)))))))),
      },
  };

  for (auto& [l1, l2, ans] : test_cases) {
    assert(is_same(Solution().addTwoNumbers(l1, l2), ans));
  }
}