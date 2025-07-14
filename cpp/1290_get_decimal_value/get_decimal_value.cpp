
// Definition for singly-linked list.
#include <cassert>
#include <tuple>
#include <vector>

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};


class Solution {
public:
    int getDecimalValue(ListNode* head) {
        ListNode* cur = head;
        int ans = 0;
        while (cur != nullptr) {
            ans = ans * 2 + cur->val;
            cur = cur->next;
        }
        return ans;
    }
};

int main() {
  std::vector<std::tuple<ListNode*, int>> tests{
      {new ListNode(1, new ListNode(0, new ListNode(1))), 5},
      {new ListNode(0), 0},
      {new ListNode(1), 1},
      {
          new ListNode(
              1,
              new ListNode(
                  0,
                  new ListNode(
                      0,
                      new ListNode(
                          1, new ListNode(
                                 0, new ListNode(
                                        0, new ListNode(
                                               1, new ListNode(
                                                      1, new ListNode(
                                                             1, new ListNode(
                                                                    0, new ListNode(
                                                                           0, new ListNode(
                                                                                  0, new ListNode(
                                                                                         0, new ListNode(
                                                                                                0, new ListNode(
                                                                                                       0))))))))))))))),
          18880,
      },
      {new ListNode(0, new ListNode(0)), 0},
  };

  for (auto &[test, expected] : tests) {
    assert(Solution().getDecimalValue(test) == expected);
  }
}