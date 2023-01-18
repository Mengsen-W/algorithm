/*
 * @Date: 2021-08-31 14:23:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-31 14:27:29
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> corpFlightBookings(vector<vector<int>>& bookings, int n) {
    vector<int> nums(n);
    for (auto& booking : bookings) {
      nums[booking[0] - 1] += booking[2];
      if (booking[1] < n) {
        nums[booking[1]] -= booking[2];
      }
    }
    for (int i = 1; i < n; i++) {
      nums[i] += nums[i - 1];
    }
    return nums;
  }
};

int main() {
  {
    vector<vector<int>> bookings{{1, 2, 10}, {2, 3, 20}, {2, 5, 25}};
    int n = 5;
    vector<int> answer{10, 55, 45, 25, 25};
    assert(Solution{}.corpFlightBookings(bookings, n) == answer);
  }
  {
    vector<vector<int>> bookings{{1, 2, 10}, {2, 2, 15}};
    int n = 2;
    vector<int> answer{10, 25};
    assert(Solution{}.corpFlightBookings(bookings, n) == answer);
  }
}