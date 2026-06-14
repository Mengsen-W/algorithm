#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  int pairSum(ListNode* head) {
    ListNode* slow = head;
    ListNode* fast = head->next;
    while (fast->next) {
      slow = slow->next;
      fast = fast->next->next;
    }
    // 反转链表
    ListNode* last = slow->next;
    while (last->next) {
      ListNode* cur = last->next;
      last->next = cur->next;
      cur->next = slow->next;
      slow->next = cur;
    }
    int ans = 0;
    ListNode* x = head;
    ListNode* y = slow->next;
    while (y) {
      ans = max(ans, x->val + y->val);
      x = x->next;
      y = y->next;
    }
    return ans;
  }
};

int main() {
  vector<tuple<ListNode*, int>> tests{
      {new ListNode(5, new ListNode(4, new ListNode(2, new ListNode(1)))), 6},
      {new ListNode(4, new ListNode(2, new ListNode(2, new ListNode(3)))), 7},
      {new ListNode(1, new ListNode(100000)), 100001},
  };

  for (auto [head, expected] : tests) {
    assert(Solution().pairSum(head) == expected);
  }
}