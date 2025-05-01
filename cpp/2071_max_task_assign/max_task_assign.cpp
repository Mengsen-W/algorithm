#include <cassert>
#include <deque>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxTaskAssign(vector<int>& tasks, vector<int>& workers, int pills, int strength) {
    int n = tasks.size(), m = workers.size();
    sort(tasks.begin(), tasks.end());
    sort(workers.begin(), workers.end());

    auto check = [&](int mid) -> bool {
      int p = pills;
      deque<int> ws;
      int ptr = m - 1;
      // 从大到小枚举每一个任务
      for (int i = mid - 1; i >= 0; --i) {
        while (ptr >= m - mid && workers[ptr] + strength >= tasks[i]) {
          ws.push_front(workers[ptr]);
          --ptr;
        }
        if (ws.empty()) {
          return false;
        }
        // 如果双端队列中最大的元素大于等于 tasks[i]
        else if (ws.back() >= tasks[i]) {
          ws.pop_back();
        } else {
          if (!p) {
            return false;
          }
          --p;
          ws.pop_front();
        }
      }
      return true;
    };

    int left = 1, right = min(m, n), ans = 0;
    while (left <= right) {
      int mid = (left + right) / 2;
      if (check(mid)) {
        ans = mid;
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, int, int>> tests{
      {{3, 2, 1}, {0, 3, 3}, 1, 1, 3},
      {{5, 4}, {0, 0, 0}, 1, 5, 1},
      {{10, 15, 30}, {0, 10, 10, 10, 10}, 3, 10, 2},
      {{5, 9, 8, 5, 9}, {1, 6, 4, 2, 6}, 1, 5, 3},
  };

  for (auto &[tasks, workers, pills, strength, ans] : tests) {
    assert(Solution().maxTaskAssign(tasks, workers, pills, strength) == ans);
  }
}