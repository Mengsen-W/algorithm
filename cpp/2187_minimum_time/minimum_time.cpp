#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  long long minimumTime(vector<int>& time, int totalTrips) {
    // 判断 t 时间内是否可以完成 totalTrips 趟旅途
    auto check = [&](long long t) -> bool {
      long long cnt = 0;
      for (int period : time) {
        cnt += t / period;
      }
      return cnt >= totalTrips;
    };

    // 二分查找下界与上界
    long long l = 1;
    long long r = (long long)totalTrips * *max_element(time.begin(), time.end());
    // 二分查找寻找满足要求的最小的 t
    while (l < r) {
      long long mid = l + (r - l) / 2;
      if (check(mid)) {
        r = mid;
      } else {
        l = mid + 1;
      }
    }
    return l;
  }
};

int main() {
  vector<tuple<vector<int>, int, long long>> tests{
      {{1, 2, 3}, 5, 3},
      {{2}, 1, 2},
  };

  for (auto& [time, totalTrips, ans] : tests) {
    assert(Solution().minimumTime(time, totalTrips) == ans);
  }
}