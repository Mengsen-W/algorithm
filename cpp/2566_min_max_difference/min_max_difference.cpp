#include <algorithm>
#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minMaxDifference(int num) {
    string s = to_string(num);
    string t = s;
    size_t pos = s.find_first_not_of('9');
    if (pos != string::npos) {
      char a = s[pos];
      replace(s.begin(), s.end(), a, '9');
    }
    char b = t[0];
    replace(t.begin(), t.end(), b, '0');
    return stoi(s) - stoi(t);
  }
};

int main() {
  vector<tuple<int, int>> tests{
      {11891, 99009},
      {90, 99},
  };

  for (auto &[num, ans] : tests) {
    assert(Solution().minMaxDifference(num) == ans);
  }
}