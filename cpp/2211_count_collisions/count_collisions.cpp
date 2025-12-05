#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int countCollisions(string directions) {
    int res = 0;
    int flag = -1;
    for (auto c : directions) {
      if (c == 'L') {
        if (flag >= 0) {
          res += flag + 1;
          flag = 0;
        }
      } else if (c == 'S') {
        if (flag > 0) {
          res += flag;
        }
        flag = 0;
      } else {
        if (flag >= 0) {
          flag++;
        } else {
          flag = 1;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"RLRSLL", 5},
      {"LLRR", 0},
  };

  for (auto [str, expect] : tests) {
    assert(Solution().countCollisions(str) == expect);
  }
}