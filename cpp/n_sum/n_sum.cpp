/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-09 15:20:25
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-09 17:38:19
 */

#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

vector<int> two_sum_v1(vector<int>& nums, int target) {
  sort(nums.begin(), nums.end());
  int left = 0, right = nums.size() - 1;
  while (left < right) {
    int sum = nums[left] + nums[right];
    if (sum < target)
      ++left;
    else if (sum > target)
      --right;
    else
      return {nums[left], nums[right]};
  }
  return {};
}

vector<vector<int>> two_sum_v2(vector<int>& nums, int target) {
  sort(nums.begin(), nums.end());
  vector<vector<int>> res;
  int left = 0, right = nums.size() - 1;
  while (left < right) {
    int sum = nums[left] + nums[right];
    int low = nums[left], hight = nums[right];
    if (sum < target) {
      while (left < right && nums[left] == low) ++left;
      ++left;
    } else if (sum > target) {
      --right;
      while (left < right && nums[right] == hight) ++right;
    } else {
      res.push_back({left, left});
      while (left < right && nums[left] == low) ++left;
      while (left < right && nums[right] == hight) ++right;
    }
  }
  return res;
}

vector<vector<int>> two_sum_v3(vector<int>& nums, int start, int target) {
  sort(nums.begin(), nums.end());
  vector<vector<int>> res;
  int left = start, right = nums.size() - 1;
  while (left < right) {
    int sum = nums[left] + nums[right];
    int low = nums[left], hight = nums[right];
    if (sum < target) {
      ++left;
      while (left < right && nums[left] == low) ++left;
    } else if (sum > target) {
      --right;
      while (left < right && nums[right] == hight) ++right;
    } else {
      res.push_back({left, left});
      while (left < right && nums[left] == low) ++left;
      while (left < right && nums[right] == hight) ++right;
    }
  }
  return res;
}

vector<vector<int>> thread_sum_v1(vector<int>& nums, int target) {
  sort(nums.begin(), nums.end());
  int size = nums.size();
  vector<vector<int>> res;
  for (int i = 0; i < size; ++i) {
    vector<vector<int>> tuples = two_sum_v3(nums, i + 1, target - nums[i]);
    // check condition
    for (vector<int>& tuple : tuples) {
      tuple.push_back(nums[i]);
      res.push_back(tuple);
    }
    while (i < size - 1 && nums[i] == nums[i + 1]) ++i;
  }

  return res;
}

vector<vector<int>> three_sum_v2(vector<int>& nums, int start, int target) {
  sort(nums.begin(), nums.end());
  int size = nums.size();
  vector<vector<int>> res;
  for (int i = start; i < size; ++i) {
    vector<vector<int>> tuples = two_sum_v3(nums, i + 1, target - nums[i]);
    // check condition
    for (vector<int>& tuple : tuples) {
      tuple.push_back(nums[i]);
      res.push_back(tuple);
    }
    while (i < size - 1 && nums[i] == nums[i + 1]) ++i;
  }

  return res;
}

vector<vector<int>> three_sum_v3(vector<int>& nums, int start, int target) {
  sort(nums.begin(), nums.end());
  int size = nums.size();
  vector<vector<int>> res;
  for (int i = start; i < size; ++i) {
    vector<vector<int>> tuples = two_sum_v3(nums, i + 1, target - nums[i]);
    // check condition
    for (vector<int>& tuple : tuples) {
      tuple.push_back(nums[i]);
      res.push_back(tuple);
    }
    while (i < size - 1 && nums[i] == nums[i + 1]) ++i;
  }

  return res;
}

vector<vector<int>> four_sum(vector<int>& nums, int start, int target) {
  sort(nums.begin(), nums.end());
  int size = nums.size();
  vector<vector<int>> res;
  for (int i = start; i < size; ++i) {
    vector<vector<int>> triples = three_sum_v2(nums, i + 1, target - nums[i]);
    for (vector<int>& triple : triples) {
      triple.push_back(nums[i]);
      res.push_back(triple);
    }
    while (i < size - 1 && nums[i] == nums[i + 1]) ++i;
  }

  return res;
}

vector<vector<int>> n_sum(vector<int>& nums, int n, int start, int target) {
  int size = nums.size();
  vector<vector<int>> res;
  if (n < 2 || size < n) return res;

  if (n == 2) {
    int low = start, high = size - 1;
    while (low < high) {
      int sum = nums[low] + nums[high], left = nums[low], right = nums[high];
      if (sum < target)
        while (low < high && nums[low] == left) ++low;
      else if (sum > target)
        while (low < high && nums[high] == right) --high;
      else {
        res.push_back({left, right});
        while (low < high && nums[low] == left) ++low;
        while (low < high && nums[high] == right) --high;
      }
    }
  } else {
    for (int i = start; i < size; ++i) {
      vector<vector<int>> sub = n_sum(nums, n - 1, i + 1, target - nums[i]);
      for (vector<int>& arr : sub) {
        arr.push_back(nums[i]);
        res.push_back(arr);
      }
      while (i < size - 1 && nums[i] == nums[i + 1]) ++i;
    }
  }

  return res;
}

int main() {
  vector<int> nums = {1, 2, 2, 3, 4, 6};
  vector<vector<int>> res = n_sum(nums, 5, 0, 14);
  for (vector<int>& arr : res) {
    for (int i : arr) {
      std::cout << i << ", ";
    }
    std::cout << std::endl;
  }
  return 0;
}