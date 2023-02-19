/*
 * @Date: 2023-02-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-19
 * @FilePath: /algorithm/cpp/1792_max_average_ratio/max_average_ratio.cpp
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  struct Ratio {
    int pass, total;
    bool operator<(const Ratio& oth) const {
      return static_cast<long long>(oth.total + 1) * oth.total * (total - pass) <
             static_cast<long long>(total + 1) * total * (oth.total - oth.pass);
    }
  };

  double maxAverageRatio(vector<vector<int>>& classes, int extraStudents) {
    priority_queue<Ratio> q;
    for (auto& c : classes) {
      q.push({c[0], c[1]});
    }

    for (int i = 0; i < extraStudents; i++) {
      auto [pass, total] = q.top();
      q.pop();
      q.push({pass + 1, total + 1});
    }

    double res = 0;
    for (size_t i = 0; i < classes.size(); i++) {
      auto [pass, total] = q.top();
      q.pop();
      res += 1.0 * pass / total;
    }
    return res / classes.size();
  }
};

int main() {
  {
    vector<vector<int>> classes{{1, 2}, {3, 5}, {2, 2}};
    int extraStudents = 2;
    double ans = 0.7833;
    assert(Solution().maxAverageRatio(classes, extraStudents) == ans);
  }

  {
    vector<vector<int>> classes{{2, 4}, {3, 9}, {4, 5}, {2, 10}};
    int extraStudents = 4;
    double ans = 0.53485;
    assert(Solution().maxAverageRatio(classes, extraStudents) == ans);
  }
}