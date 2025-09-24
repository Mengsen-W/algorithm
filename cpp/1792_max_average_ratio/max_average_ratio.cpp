#include <cassert>
#include <cmath>
#include <queue>
#include <tuple>
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
  vector<tuple<vector<vector<int>>, int, double>> tests{
      {{{1, 2}, {3, 5}, {2, 2}}, 2, 0.78333},
      {{{2, 4}, {3, 9}, {4, 5}, {2, 10}}, 4, 0.53485},
  };

  for (auto& [classes, extraStudents, expect] : tests) {
    double res = Solution().maxAverageRatio(classes, extraStudents);
    assert(fabs(res - expect) < 1e-5);
  }
}