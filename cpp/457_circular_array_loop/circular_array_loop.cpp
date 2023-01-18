/*
 * @Date: 2021-08-07 12:31:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-07 14:07:27
 */

#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
 public:
  bool circularArrayLoop_double_pointer(vector<int>& nums) {
    int n = nums.size();
    auto next = [&](int cur) { return ((cur + nums[cur]) % n + n) % n; };
    for (int i = 0; i < n; i++) {
      if (nums[i] == 0) continue;

      int slow = i, fast = next(i);
      // 判断非零且方向相同
      while (nums[slow] * nums[fast] > 0 && nums[slow] * nums[next(fast)] > 0) {
        if (slow == fast) {
          if (slow != next(slow)) {
            return true;
          } else {
            break;
          }
        }
        slow = next(slow);
        fast = next(next(fast));
      }
      int add = i;
      while (nums[add] * nums[next(add)] > 0) {
        int tmp = add;
        add = next(add);
        nums[tmp] = 0;
      }
    }
    return false;
  }

  bool circularArrayLoop_bfs(vector<int>& nums) {
    const int OFFSET = 100010;
    int n = nums.size();
    for (int i = 0; i < n; i++) {
      if (nums[i] >= OFFSET) continue;
      int cur = i, tag = OFFSET + i, last = -1;
      bool flag = nums[cur] > 0;
      while (true) {
        int next = ((cur + nums[cur]) % n + n) % n;
        last = nums[cur];
        nums[cur] = tag;
        cur = next;
        if (cur == i) break;
        if (nums[cur] >= OFFSET) break;
        if (flag && nums[cur] < 0) break;
        if (!flag && nums[cur] > 0) break;
      }
      if (last % n != 0 && nums[cur] == tag) return true;
    }
    return false;
  }
};

int main() {
  {
    vector<int> nums{2, -1, 1, 2, 2};
    assert(Solution{}.circularArrayLoop_double_pointer(nums));
    // bfs修改了nums
    assert(Solution{}.circularArrayLoop_bfs(nums));
  }
  {
    vector<int> nums{-1, 2};
    assert(!Solution{}.circularArrayLoop_bfs(nums));
    assert(!Solution{}.circularArrayLoop_double_pointer(nums));
  }
  {
    vector<int> nums{2, -1, 1, -2, -2};
    assert(!Solution{}.circularArrayLoop_bfs(nums));
    assert(!Solution{}.circularArrayLoop_double_pointer(nums));
  }
  return 0;
}