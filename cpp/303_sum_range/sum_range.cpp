/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-01 20:29:40
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-01 20:36:50
 */

#include <cassert>
#include <vector>

using namespace std;

class num_array {
 public:
  num_array(vector<int>&& nums) {
    int n = nums.size();
    sums.resize(n + 1);
    for (int i = 0; i < n; i++) {
      sums[i + 1] = sums[i] + nums[i];
    }
  }
  int sum_range(int i, int j) { return sums[j + 1] - sums[i]; }

 private:
  vector<int> sums;
};

int main() {
  num_array array{vector<int>{-2, 0, 3, -5, 2, -1}};
  assert(array.sum_range(0, 2) == 1);
  assert(array.sum_range(2, 5) == -1);
  assert(array.sum_range(0, 5) == -3);
  return 0;
}