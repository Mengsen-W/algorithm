#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string largestGoodInteger(string num) {
    int n = num.size();
    string res;
    for (int i = 0; i < n - 2; ++i) {
      if (num[i] == num[i + 1] && num[i + 1] == num[i + 2]) {
        res = max(res, num.substr(i, 3));
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"6777133339", "777"},
      {"2300019", "000"},
      {"42352338", ""},
  };

  for (auto &[num, ans] : tests) {
    assert(Solution().largestGoodInteger(num) == ans);
  }
}