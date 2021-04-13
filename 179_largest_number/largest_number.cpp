/*
 * @Date: 2021-04-12 08:55:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-12 09:06:21
 */

#include <algorithm>
#include <cassert>
#include <string>
#include <vector>

using namespace std;

string largest_number(vector<int>& nums) {
  sort(nums.begin(), nums.end(), [](const int& a, const int& b) {
    return to_string(a) + to_string(b) > to_string(b) + to_string(a);
  });  //这一步的cmp函数是关键

  string res;
  for (int i : nums) res += to_string(i);  //排完序就可以直接遍历相加了

  if (res[0] == '0') return "0";  //判断全为0的特殊情况
  return res;
}

int main() {
  {
    vector<int> nums{10, 2};
    assert(largest_number(nums) == "210");
  }
  {
    vector<int> nums{3, 30, 34, 5, 9};
    assert(largest_number(nums) == "9534330");
  }
  {
    vector<int> nums{1};
    assert(largest_number(nums) == "1");
  }
  {
    vector<int> nums{10};
    assert(largest_number(nums) == "10");
  }
  return 0;
}