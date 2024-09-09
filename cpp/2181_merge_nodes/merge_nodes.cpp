

// Definition for singly-linked list.
#include <cassert>
#include <tuple>
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
  ListNode* mergeNodes(ListNode* head) {
    ListNode* dummy = new ListNode();
    ListNode* tail = dummy;

    int total = 0;
    for (ListNode* cur = head->next; cur; cur = cur->next) {
      if (cur->val == 0) {
        ListNode* node = new ListNode(total);
        tail->next = node;
        tail = tail->next;
        total = 0;
      } else {
        total += cur->val;
      }
    }

    return dummy->next;
  }
};

vector<int> list_to_vec(ListNode* l) {
  ListNode* step = l;
  vector<int> ans{};
  while (step != nullptr) {
    ans.push_back(step->val);
    step = step->next;
  }
  return ans;
}

int main() {
  vector<tuple<ListNode*, ListNode*>> tests{
      {
          new ListNode(
              0, new ListNode(
                     3, new ListNode(
                            1, new ListNode(0, new ListNode(4, new ListNode(5, new ListNode(2, new ListNode(0)))))))),
          new ListNode(4, new ListNode(11)),
      },

      {
          new ListNode(
              0, new ListNode(
                     1, new ListNode(
                            0, new ListNode(3, new ListNode(0, new ListNode(2, new ListNode(2, new ListNode(0)))))))),
          new ListNode(1, new ListNode(3, new ListNode(4))),
      },

  };

  for (auto& [head, ans] : tests) {
    assert(list_to_vec(Solution().mergeNodes(head)) == list_to_vec(ans));
  }
}