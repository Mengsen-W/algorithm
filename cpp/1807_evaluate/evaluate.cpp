#include <cassert>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  string evaluate(string s, vector<vector<string>>& knowledge) {
    unordered_map<string, string> dict;
    for (auto& kd : knowledge) {
      dict[kd[0]] = kd[1];
    }
    bool addKey = false;
    string key, res;
    for (char c : s) {
      if (c == '(') {
        addKey = true;
      } else if (c == ')') {
        if (dict.count(key) > 0) {
          res += dict[key];
        } else {
          res.push_back('?');
        }
        addKey = false;
        key.clear();
      } else {
        if (addKey) {
          key.push_back(c);
        } else {
          res.push_back(c);
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<string, vector<vector<string>>, string>> tests{
      {"(name)is(age)yearsold", {{"name", "bob"}, {"age", "two"}}, "bobistwoyearsold"},
      {"hi(name)", {{"a", "b"}}, "hi?"},
      {"(a)(a)(a)aaa", {{"a", "yes"}}, "yesyesyesaaa"},
  };

  for (auto& [s, knowledge, ans] : tests) {
    assert(Solution().evaluate(s, knowledge) == ans);
  }
}
