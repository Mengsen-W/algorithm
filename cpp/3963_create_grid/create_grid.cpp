#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
public:
    vector<string> createGrid(int m, int n) {
      vector<string> ans(m);
      ans[0].resize(n, '.');
      for (int i = 1; i < m; i++) {
        ans[i].resize(n, '#');
        ans[i][n - 1] = '.';
      }
        return ans;
    }
};

int main() {
  vector<tuple<int, int, vector<string>>> tests{
      {2, 3, {"..#", "#.."}},
      {3, 3, {"..#", "#..", "##."}},
      {1, 4, {"...."}},
  };

  for (auto [m, n, expected] : tests) {
    assert(Solution().createGrid(m, n) == expected);
  }
}