/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 17:16:24
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-19 18:57:36
 */

#include <algorithm>
#include <functional>
#include <vector>

std::vector<std::vector<int>>& premute(std::vector<int>& nums) {
  // 记录结果
  std::vector<std::vector<int>> res{};
  // 记录回溯过程中的状态
  std::vector<int> trace{};

  // 回溯函数
  std::function<void()> backtrace = [&backtrace, &res, &nums,
                                     &trace]() -> void {
    // 触发结果加入
    if (trace.size() == nums.size()) {
      res.push_back(trace);
      return;
    }

    for (int i = 0; i < nums.size(); ++i) {
      if (trace.end() == std::find(trace.begin(), trace.end(), nums[i]))
        continue;
      trace.push_back(nums[i]);
      backtrace();
      trace.pop_back();
    }
  };

  // 调用过程
  backtrace();
  return res;
}

int main() { return 0; }