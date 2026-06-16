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
  ListNode* deleteMiddle(ListNode* head) {
    if (head->next == nullptr) {
      return nullptr;
    }

    ListNode* slow = head;
    ListNode* fast = head;
    ListNode* pre = nullptr;
    while (fast && fast->next) {
      fast = fast->next->next;
      pre = slow;
      slow = slow->next;
    }
    pre->next = pre->next->next;
    return head;
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
  vector<tuple<ListNode*, ListNode*>> tests{
      {
          new ListNode{
              1,
              new ListNode{
                  3,
                  new ListNode{
                      4,
                      new ListNode{
                          7,
                          new ListNode{
                              1,
                              new ListNode{
                                  2,
                                  new ListNode{
                                      6,
                                  },
                              },
                          },
                      },
                  },
              },
          },
          new ListNode{
              1,
              new ListNode{
                  3,
                  new ListNode{
                      4,
                      new ListNode{
                          1,
                          new ListNode{
                              2,
                              new ListNode{
                                  6,
                              },
                          },
                      },
                  },
              },
          },
      },
      {
          new ListNode{
              1,
              new ListNode{
                  2,
                  new ListNode{
                      3,
                      new ListNode{
                          4,
                      },
                  },
              },
          },
          new ListNode{
              1,
              new ListNode{
                  2,
                  new ListNode{
                      4,
                  },
              },
          },
      },
      {
          new ListNode{
              2,
              new ListNode{
                  1,
              },
          },
          new ListNode{
              2,
          },
      },
  };

  for (auto& [head, ans] : tests) {
    assert(is_same_list(Solution().deleteMiddle(head), ans));
  }
}