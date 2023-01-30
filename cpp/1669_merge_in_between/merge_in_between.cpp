/*
 * @Date: 2023-01-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-30
 * @FilePath: /algorithm/cpp/1669_merge_in_between/merge_in_between.cpp
 */

#include <cassert>
#include <cstddef>
#include <iostream>

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
    ListNode* preA = list1;
    for (int i = 0; i < a - 1; i++) {
      preA = preA->next;
    }
    ListNode* preB = preA;
    for (int i = 0; i < b - a + 2; i++) {
      preB = preB->next;
    }
    preA->next = list2;
    while (list2->next != nullptr) {
      list2 = list2->next;
    }
    list2->next = preB;
    return list1;
  }
};

bool CompareLists(ListNode* headA, ListNode* headB) {
  ListNode* p = headA;
  ListNode* q = headB;
  while (p != nullptr && q != nullptr && p->val == q->val) {
    p = p->next;
    q = q->next;
  }
  return p == q ? true : false;
}

int main() {
  {
    ListNode* list1 =
        new ListNode{0, new ListNode{1, new ListNode{2, new ListNode{3, new ListNode{4, new ListNode{5, nullptr}}}}}};
    int a = 3, b = 4;
    ListNode* list2 = new ListNode{1000000, new ListNode{1000001, new ListNode{1000002, nullptr}}};
    ListNode* ans = new ListNode{
        0,
        new ListNode{
            1, new ListNode{
                   2, new ListNode{1000000, new ListNode{1000001, new ListNode{1000002, new ListNode{5, nullptr}}}}}}};
    assert(CompareLists(Solution().mergeInBetween(list1, a, b, list2), ans));
  }

  {
    ListNode* list1 = new ListNode{
        0,
        new ListNode{1, new ListNode{2, new ListNode{3, new ListNode{4, new ListNode{5, new ListNode{6, nullptr}}}}}}};
    int a = 2, b = 5;
    ListNode* list2 = new ListNode{
        1000000, new ListNode{1000001, new ListNode{1000002, new ListNode{1000003, new ListNode{1000004, nullptr}}}}};
    ListNode* ans = new ListNode{
        0,
        new ListNode{
            1,
            new ListNode{
                1000000,
                new ListNode{
                    1000001,
                    new ListNode{1000002, new ListNode{1000003, new ListNode{1000004, new ListNode{6, nullptr}}}}}}}};
    assert(CompareLists(Solution().mergeInBetween(list1, a, b, list2), ans));
  }
}