#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string processStr(string s) {
    string result = "";
    for (auto it : s) {
      if (it == '*') {
        if (result.size()) {
          result.pop_back();
        }
      } else if (it == '#') {
        result += result;
      } else if (it == '%') {
        result = string(result.rbegin(), result.rend());
      } else {
        result += it;
      }
    }
    return result;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"a#b%*", "ba"},
      {"z*#", ""},
  };

  for (auto& [s, ans] : tests) {
    assert(Solution().processStr(s) == ans);
  }
}