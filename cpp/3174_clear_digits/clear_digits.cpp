#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string clearDigits(string s) {
    string res;
    for (char c : s) {
      if (isdigit(c)) {
        res.pop_back();
      } else {
        res.push_back(c);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"abc", "abc"},
      {"cb34", ""},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().clearDigits(s) == ans);
  }
}