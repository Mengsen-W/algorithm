/*
 * @Date: 2021-08-30 12:49:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-30 13:10:20
 * @FilePath: /algorithm/528_pick_index/pick_index.cpp
 * @Description: file content
 */

#include <cassert>
#include <random>
#include <vector>

using namespace std;

class Solution {
 private:
  mt19937 gen;
  uniform_int_distribution<int> dis;
  vector<int> pre;

 public:
  Solution(vector<int>& w)
      : gen(random_device{}()), dis(1, accumulate(w.begin(), w.end(), 0)) {
    partial_sum(w.begin(), w.end(), back_inserter(pre));
  }

  int pickIndex() {
    int x = dis(gen);
    return lower_bound(pre.begin(), pre.end(), x) - pre.begin();
  }
};

int main() {
  {
    vector<int> nums{1};
    Solution s{nums};
    assert(s.pickIndex() == 0);
  }
  {
    vector<int> nums{1, 3};
    Solution s{nums};
    assert(s.pickIndex() == 1);
  }
}