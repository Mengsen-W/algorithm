#include <cassert>
#include <queue>
#include <stack>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long countNonDecreasingSubarrays(vector<int>& nums, int k) {
    int n = nums.size();
    long long ans = 0;
    int operations = 0;
    // 优先队列维护窗口最大值
    priority_queue<pair<int, int>> q;
    // 记录被该下标覆盖的元素集合
    vector<vector<int>> cnt(n);
    // 右侧第一个大于等于的下标
    vector<int> pos(n, n);
    // 递减单调栈
    stack<int> st;

    for (int left = 0, right = 0; right < n; right++) {
      // 维护单调栈
      while (!st.empty() && nums[right] >= nums[st.top()]) {
        pos[st.top()] = right;
        st.pop();
      }
      // 栈顶即为 nums[right] 的左边最近的更大元素
      if (!st.empty()) {
        cnt[st.top()].push_back(right);
      }
      st.push(right);

      // 优先队列队首元素即为窗口最大值
      q.emplace(nums[right], right);
      // 窗口加入 nums[right] 操作次数需要增加
      operations += max(0, q.top().first - nums[right]);

      // 最小操作次数超过 k，此时需要缩小窗口
      while (operations > k) {
        // 移除窗口左侧元素 nums[left]，操作次数需要较少
        for (int i : cnt[left]) {
          // 移除贡献
          operations -= (nums[left] - nums[i]) * (min(pos[i], right + 1) - i);
        }
        left++;
      }

      // 从优先队列中移除小于下标 left 的元素
      while (!q.empty() && q.top().second < left) {
        q.pop();
      }

      // 以 right 为结尾的合法连续子数组数目为 right - left + 1
      ans += right - left + 1;
    }

    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{6, 3, 1, 2, 4, 4}, 7, 17},
      {{6, 3, 1, 3, 6}, 4, 12},
  };

  for (auto [nums, k, expected] : tests) {
    assert(Solution().countNonDecreasingSubarrays(nums, k) == expected);
  }
}