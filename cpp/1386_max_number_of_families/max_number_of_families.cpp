#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxNumberOfFamilies(int n, vector<vector<int>>& reservedSeats) {
    int left = 0b11110000;
    int middle = 0b11000011;
    int right = 0b00001111;

    unordered_map<int, int> occupied;
    for (const vector<int>& seat : reservedSeats) {
      if (seat[1] >= 2 && seat[1] <= 9) {
        occupied[seat[0]] |= (1 << (seat[1] - 2));
      }
    }

    int ans = (n - occupied.size()) * 2;
    for (auto& [row, bitmask] : occupied) {
      if (((bitmask | left) == left) || ((bitmask | middle) == middle) || ((bitmask | right) == right)) {
        ++ans;
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {3, {{1, 2}, {1, 3}, {1, 8}, {2, 6}, {3, 1}, {3, 10}}, 4},
      {2, {{2, 1}, {1, 8}, {2, 6}}, 2},
      {4, {{4, 3}, {1, 4}, {4, 6}, {1, 7}}, 4},
  };

  for (auto& [n, seats, expected] : tests) {
    assert(Solution().maxNumberOfFamilies(n, seats) == expected);
  }
  return 0;
}