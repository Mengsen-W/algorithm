/*
 * @Date: 2021-07-21 18:18:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-21 18:26:04
 */

#include <cassert>

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
    if (headA == nullptr || headB == nullptr) {
      return nullptr;
    }
    ListNode *pA = headA, *pB = headB;
    while (pA != pB) {
      pA = pA == nullptr ? headB : pA->next;
      pB = pB == nullptr ? headA : pB->next;
    }
    return pA;
  }
};

int main() {
  {
    ListNode *intersection =
        new ListNode{8, new ListNode{4, new ListNode{5, nullptr}}};
    ListNode *headA = new ListNode{4, new ListNode{1, intersection}};
    ListNode *headB =
        new ListNode{5, new ListNode{0, new ListNode{1, intersection}}};
    assert(Solution{}.getIntersectionNode(headA, headB)->val == 8);
  }
  {
    ListNode *intersection =
        new ListNode{2, new ListNode{4, nullptr}};
    ListNode *headA = new ListNode{0, new ListNode{9, new ListNode{1, intersection}}};
    ListNode *headB =
        new ListNode{3,  intersection};
    assert(Solution{}.getIntersectionNode(headA, headB)->val == 2);
  }
  {
    ListNode *headA = new ListNode{2, new ListNode{6, new ListNode{4, nullptr}}};
    ListNode *headB =
        new ListNode{1,  new ListNode{5, nullptr}};
    assert(Solution{}.getIntersectionNode(headA, headB) == nullptr);
  }
}