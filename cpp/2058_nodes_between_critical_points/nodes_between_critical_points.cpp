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
  vector<int> nodesBetweenCriticalPoints(ListNode* head) {
    int minDist = -1, maxDist = -1;
    int first = -1, last = -1, pos = 0;
    ListNode* cur = head;
    while (cur->next->next) {
      // 获取连续的三个节点的值
      int x = cur->val;
      int y = cur->next->val;
      int z = cur->next->next->val;
      // 如果 y 是临界点
      if (y > max(x, z) || y < min(x, z)) {
        if (last != -1) {
          // 用相邻临界点的距离更新最小值
          minDist = (minDist == -1 ? pos - last : min(minDist, pos - last));
          // 用到第一个临界点的距离更新最大值
          maxDist = max(maxDist, pos - first);
        }
        if (first == -1) {
          first = pos;
        }
        // 更新上一个临界点
        last = pos;
      }
      cur = cur->next;
      ++pos;
    }
    return {minDist, maxDist};
  }
};

int main() {
  vector<tuple<ListNode*, vector<int>>> tests{
      {new ListNode{3, new ListNode{1}}, {-1, -1}},
      {new ListNode{
           5, new ListNode{3, new ListNode{1, new ListNode{2, new ListNode{5, new ListNode{1, new ListNode{2}}}}}}},
       {1, 3}},
      {new ListNode{
           1,
           new ListNode{
               3,
               new ListNode{
                   2,
                   new ListNode{
                       2, new ListNode{3, new ListNode{2, new ListNode{2, new ListNode{2, new ListNode{7}}}}}}}}},
       {3, 3}},
      {new ListNode{2, new ListNode{3, new ListNode{3, new ListNode{2}}}}, {-1, -1}},
  };

  for (auto& [head, ans] : tests) {
    assert(Solution().nodesBetweenCriticalPoints(head) == ans);
  }
}