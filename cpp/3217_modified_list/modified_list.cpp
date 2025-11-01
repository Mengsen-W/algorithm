#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  ListNode* modifiedList(vector<int>& nums, ListNode* head) {
    unordered_map<int, int> isExist;
    for (int num : nums) {
      isExist[num] = 1;
    }
    ListNode sentry(0, head);
    ListNode* p = &sentry;
    while (p->next) {
      if (isExist[p->next->val]) {
        p->next = p->next->next;
      } else {
        p = p->next;
      }
    }
    return sentry.next;
  }
};

bool is_same_list(ListNode* l1, ListNode* l2) {
  while (l1 != nullptr && l2 != nullptr) {
    if (l1->val != l2->val) {
      return false;
    }
    l1 = l1->next;
    l2 = l2->next;
  }

  // 如果其中一个链表遍历完了，但另一个链表还没有遍历完，那么它们不相等。
  if (l1 != nullptr || l2 != nullptr) {
    return false;
  }

  return true;
}

int main() {
  vector<tuple<vector<int>, ListNode*, ListNode*>> tests{
      {
          {1,2,3},
          new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5))))),
          new ListNode(4, new ListNode(5)),
      },
      {
          {1},
          new ListNode(1, new ListNode(2, new ListNode(1, new ListNode(2, new ListNode(1, new ListNode(2)))))),
          new ListNode(2, new ListNode(2, new ListNode(2))),
      },
      {
          {5},
          new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4)))),
          new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4)))),
      },
  };

  for (auto& [nums, head, expected] : tests) {
    assert(is_same_list(expected, Solution().modifiedList(nums, head)));
  }
}
