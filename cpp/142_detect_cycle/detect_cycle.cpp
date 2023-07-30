/*
 * @Date: 2023-07-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-30
 * @FilePath: /algorithm/cpp/142_detect_cycle/detect_cycle.cpp
 */

#include <cassert>
#include <unordered_set>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode *detectCycle(ListNode *head) {
    unordered_set<ListNode *> visited;
    while (head != nullptr) {
      if (visited.count(head)) {
        return head;
      }
      visited.insert(head);
      head = head->next;
    }
    return nullptr;
  }
};

int main() {
  {
    ListNode *head = new ListNode(3, new ListNode(2, new ListNode(0, new ListNode(-4))));
    head->next->next->next->next = head->next;
    assert(Solution().detectCycle(head)->val == 2);
  }

  {
    ListNode *head = new ListNode(1, new ListNode(2));
    head->next = head;
    assert(Solution().detectCycle(head)->val == 1);
  }

  {
    ListNode *head = new ListNode(1);
    assert(Solution().detectCycle(head) == nullptr);
  }
}