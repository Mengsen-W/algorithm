#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minimumOperations(string num) {
    int n = num.length();
    bool find0 = false, find5 = false;
    for (int i = n - 1; i >= 0; --i) {
      if (num[i] == '0' || num[i] == '5') {
        if (find0) {
          return n - i - 2;
        }
        if (num[i] == '0') {
          find0 = true;
        } else {
          find5 = true;
        }
      } else if (num[i] == '2' || num[i] == '7') {
        if (find5) {
          return n - i - 2;
        }
      }
    }
    if (find0) {
      return n - 1;
    }
    return n;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"2245047", 2},
      {"2908305", 3},
      {"10", 1},
  };

  for (auto &[num, ans] : tests) {
    assert(Solution().minimumOperations(num) == ans);
  }
}