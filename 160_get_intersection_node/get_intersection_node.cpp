/*
 * @Date: 2021-06-04 19:17:24
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-06-04 20:22:33
 */

#include <cassert>
#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

struct ListNode {
  int val;
  ListNode *next;
};

ListNode *getIntersectionNode_hashset(ListNode *headA, ListNode *headB) {
  unordered_set<ListNode *> visited;
  ListNode *temp = headA;
  while (temp != nullptr) {
    visited.insert(temp);
    temp = temp->next;
  }
  temp = headB;
  while (temp != nullptr) {
    if (visited.count(temp)) {
      return temp;
    }
    temp = temp->next;
  }
  return nullptr;
}

ListNode *getIntersectionNode_double_pointer(ListNode *headA, ListNode *headB) {
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

int main() {
  {
    ListNode *intersect =
        new ListNode{8, new ListNode{4, new ListNode{5, nullptr}}};
    ListNode *a = new ListNode{4, new ListNode{1, intersect}};
    ListNode *b = new ListNode{5, new ListNode{0, new ListNode{1, intersect}}};
    assert(getIntersectionNode_double_pointer(a, b)->val == 8);
    assert(getIntersectionNode_hashset(a, b)->val == 8);
  }
  {
    ListNode *intersect = new ListNode{2, new ListNode{4, nullptr}};
    ListNode *a = new ListNode{0, new ListNode{9, new ListNode{1, intersect}}};
    ListNode *b = new ListNode{3, intersect};
    assert(getIntersectionNode_double_pointer(a, b)->val == 2);
    assert(getIntersectionNode_hashset(a, b)->val == 2);
  }
  {
    ListNode *a = new ListNode{0, new ListNode{6, new ListNode{4, nullptr}}};
    ListNode *b = new ListNode{1, new ListNode{5, nullptr}};
    assert(getIntersectionNode_double_pointer(a, b) == nullptr);
    assert(getIntersectionNode_hashset(a, b) == nullptr);
  }
  return 0;
}