use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pair {
    first: i32,
    second: i32,
}

impl Pair {
    fn new(first: i32, second: i32) -> Self {
        Pair { first, second }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Triple {
    price: i32,
    shop: i32,
    movie: i32,
}

impl Triple {
    fn new(price: i32, shop: i32, movie: i32) -> Self {
        Triple { price, shop, movie }
    }
}

struct MovieRentingSystem {
    t_price: HashMap<Pair, i32>,
    t_valid: HashMap<i32, BTreeSet<Pair>>,
    t_rent: BTreeSet<Triple>,
}

impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut t_price = HashMap::new();
        let mut t_valid = HashMap::new();

        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];
            t_price.insert(Pair::new(shop, movie), price);
            t_valid
                .entry(movie)
                .or_insert_with(BTreeSet::new)
                .insert(Pair::new(price, shop));
        }

        MovieRentingSystem {
            t_price,
            t_valid,
            t_rent: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.t_valid.get(&movie).map_or_else(Vec::new, |set| {
            set.iter().take(5).map(|p| p.second).collect()
        })
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.t_price[&Pair::new(shop, movie)];
        self.t_valid
            .get_mut(&movie)
            .unwrap()
            .remove(&Pair::new(price, shop));
        self.t_rent.insert(Triple::new(price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.t_price[&Pair::new(shop, movie)];
        self.t_valid
            .get_mut(&movie)
            .unwrap()
            .insert(Pair::new(price, shop));
        self.t_rent.remove(&Triple::new(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.t_rent
            .iter()
            .take(5)
            .map(|t| vec![t.shop, t.movie])
            .collect()
    }
}

fn main() {
    let mut movie_renting_system = MovieRentingSystem::new(
        3,
        vec![
            vec![0, 1, 5],
            vec![0, 2, 6],
            vec![0, 3, 7],
            vec![1, 1, 4],
            vec![1, 2, 7],
            vec![2, 1, 5],
        ],
    );

    assert_eq!(movie_renting_system.search(1), vec![1, 0, 2]);
    movie_renting_system.rent(0, 1);
    movie_renting_system.rent(1, 2);
    assert_eq!(movie_renting_system.report(), vec![vec![0, 1], vec![1, 2]]);
    movie_renting_system.drop(1, 2);
    assert_eq!(movie_renting_system.search(2), vec![0, 1]);
}
