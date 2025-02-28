#include <cassert>
#include <queue>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class FoodRatings {
  unordered_map<string, pair<int, string>> foodMap;
  unordered_map<string, priority_queue<pair<int, string>, vector<pair<int, string>>, greater<>>> ratingMap;
  int n;

 public:
  FoodRatings(vector<string>& foods, vector<string>& cuisines, vector<int>& ratings) {
    n = foods.size();
    for (int i = 0; i < n; ++i) {
      auto &food = foods[i], &cuisine = cuisines[i];
      int rating = ratings[i];
      foodMap[food] = {rating, cuisine};
      ratingMap[cuisine].emplace(n - rating, food);
    }
  }

  void changeRating(string food, int newRating) {
    auto& [rating, cuisine] = foodMap[food];
    ratingMap[cuisine].emplace(n - newRating, food);
    rating = newRating;
  }

  string highestRated(string cuisine) {
    auto& q = ratingMap[cuisine];
    auto& [rating, food] = q.top();
    while (n - rating != foodMap[food].first) {
      q.pop();
    }
    return q.top().second;
  }
};

int main() {
  vector<string> foods{"kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"};
  vector<string> cuisines{"korean", "japanese", "japanese", "greek", "japanese", "korean"};
  vector<int> ratings{9, 12, 8, 15, 14, 7};
  FoodRatings foodRatings = FoodRatings(foods, cuisines, ratings);
  assert(foodRatings.highestRated("korean") == "kimchi");
  assert(foodRatings.highestRated("japanese") == "ramen");
  foodRatings.changeRating("sushi", 16);
  assert(foodRatings.highestRated("japanese") == "sushi");
  foodRatings.changeRating("ramen", 16);
  assert(foodRatings.highestRated("japanese") == "ramen");
}