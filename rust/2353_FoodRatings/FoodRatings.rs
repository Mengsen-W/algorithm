use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct Pair(i32, String);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0).then_with(|| other.1.cmp(&self.1))
    }
}

#[derive(Debug)]
struct FoodRatings {
    food_map: HashMap<String, (i32, String)>,
    rating_map: HashMap<String, BinaryHeap<Pair>>,
    n: i32,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let n = foods.len() as i32;
        let mut food_map = HashMap::new();
        let mut rating_map = HashMap::new();

        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];
            food_map.insert(food.clone(), (rating, cuisine.clone()));
            rating_map
                .entry(cuisine.clone())
                .or_insert_with(BinaryHeap::new)
                .push(Pair(n - rating, food.clone()));
        }

        FoodRatings {
            food_map,
            rating_map,
            n,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some((old_rating, cuisine)) = self.food_map.get_mut(&food) {
            *old_rating = new_rating;
            self.rating_map
                .get_mut(cuisine)
                .unwrap()
                .push(Pair(self.n - new_rating, food.clone()));
        }
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        if let Some(heap) = self.rating_map.get_mut(&cuisine) {
            while let Some(&ref item) = heap.peek() {
                let rating = &item.0;
                let food = &item.1;
                if self.n - rating == self.food_map[food].0 {
                    return food.clone();
                }
                heap.pop();
            }
        }
        String::new()
    }
}

fn main() {
    let mut food_ratings = FoodRatings::new(
        vec!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
        vec![
            "korean", "japanese", "japanese", "greek", "japanese", "korean",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
        vec![9, 12, 8, 15, 14, 7],
    );

    assert_eq!(
        food_ratings.highest_rated("korean".to_string()),
        "kimchi".to_string()
    );
    assert_eq!(
        food_ratings.highest_rated("japanese".to_string()),
        "ramen".to_string()
    );
    food_ratings.change_rating("sushi".to_string(), 16);
    assert_eq!(
        food_ratings.highest_rated("japanese".to_string()),
        "sushi".to_string()
    );
    food_ratings.change_rating("ramen".to_string(), 16);
    assert_eq!(
        food_ratings.highest_rated("japanese".to_string()),
        "ramen".to_string()
    );
}
