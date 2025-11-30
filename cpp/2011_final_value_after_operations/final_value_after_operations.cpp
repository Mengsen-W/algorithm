#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int finalValueAfterOperations(vector<string>& operations) {
    int x = 0;
    for (auto& op : operations) {
      if (op == "X++" || op == "++X") {
        x++;
      } else {
        x--;
      }
    }
    return x;
  }
};

int main() {
  vector<tuple<vector<string>, int>> tests{
      {{"--X", "X++", "X++"}, 1},
      {{"++X", "++X", "X++"}, 3},
      {{"X++", "++X", "--X", "X--"}, 0},
  };

  for (auto& [operations, expected] : tests) {
    assert(Solution().finalValueAfterOperations(operations) == expected);
  }
}