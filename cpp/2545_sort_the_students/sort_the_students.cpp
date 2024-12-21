#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<vector<int>> sortTheStudents(vector<vector<int>>& score, int k) {
    sort(score.begin(), score.end(), [&](const vector<int>& u, const vector<int>& v) { return u[k] > v[k]; });
    return score;
  }
};

int main() {
  vector<tuple<vector<vector<int>>, int, vector<vector<int>>>> tests{
      {{{10, 6, 9, 1}, {7, 5, 11, 2}, {4, 8, 3, 15}}, 2, {{7, 5, 11, 2}, {10, 6, 9, 1}, {4, 8, 3, 15}}},
      {{{3, 4}, {5, 6}}, 0, {{5, 6}, {3, 4}}},
  };
  
  for (auto &[source, k, ans] : tests) {
    assert(Solution().sortTheStudents(source, k) == ans);
  }
}