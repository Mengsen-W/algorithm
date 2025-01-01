#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  string binary(int x) {
    string s;
    while (x) {
      s.push_back('0' + (x & 1));
      x >>= 1;
    }
    reverse(s.begin(), s.end());
    return s;
  }

  string convertDateToBinary(string date) {
    int year = stoi(date.substr(0, 4));
    int month = stoi(date.substr(5, 2));
    int day = stoi(date.substr(8, 2));
    return binary(year) + "-" + binary(month) + "-" + binary(day);
  }
};

int main() {
  vector<tuple<string, string>> tests{
      {"2080-02-29", "100000100000-10-11101"},
      {"1900-01-01", "11101101100-1-1"},
  };

  for (auto &[date, ans] : tests) {
    assert(Solution().convertDateToBinary(date) == ans);
  }
}