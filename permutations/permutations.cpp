/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-19 17:16:24
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-29 19:13:14
 */

#include <algorithm>
#include <functional>
#include <iostream>
#include <vector>

std::vector<std::vector<int>> premute(std::vector<int>& nums) {
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
      if (trace.end() != std::find(trace.begin(), trace.end(), nums[i]))
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

int main() {
  std::vector<int> nums = {1, 2, 3, 4, 5};
  std::vector<std::vector<int>> res = premute(nums);

  std::cout << "number = " << res.size() << std::endl;

  for (int i = 0; i < res.size(); ++i) {
    for (int j = 0; j < res[i].size(); ++j) {
      std::cout << res[i][j];
    }
    std::cout << std::endl;
  }
  return 0;
}