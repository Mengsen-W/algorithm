/*
 * @Date: 2021-12-30 01:18:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-30 01:39:11
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  bool isNStraightHand(vector<int> hand, int groupSize) {
    int n = hand.size();
    if (n % groupSize != 0) {
      return false;
    }
    sort(hand.begin(), hand.end());
    unordered_map<int, int> cnt;
    for (auto& num : hand) {
      cnt[num]++;
    }
    for (auto& x : hand) {
      if (!cnt.count(x)) {
        continue;
      }
      for (int j = 0; j < groupSize; j++) {
        int num = x + j;
        if (!cnt.count(num)) {
          return false;
        }
        cnt[num]--;
        if (cnt[num] == 0) {
          cnt.erase(num);
        }
      }
    }
    return true;
  }
};

int main() {
  assert(Solution().isNStraightHand({1, 2, 3, 6, 2, 3, 4, 7, 8}, 3) == true);
  assert(Solution().isNStraightHand({1, 2, 3, 4, 5}, 4) == false);
}