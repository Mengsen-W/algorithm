#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minPartitions(string n) {
    int res = 0;
    for (char c : n) {
      res = max(res, c - '0');
    }
    return res;
  }
};

int main() {
  vector<tuple<string, int>> tests{
      {"32", 3},
      {"82734", 8},
      {"27346209830709182346", 9},
  };

  for (auto [n, expected] : tests) {
    assert(Solution().minPartitions(n) == expected);
  }
}