// modules2.rs
// Make me compile! Scroll down for hints :)

pub mod delicious_snacks { 
    //use self::fruits::PEAR as fruit;
    //use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!("favorite snacks: {} and {}",
             delicious_snacks::fruits::PEAR,
             delicious_snacks::veggies::CUCUMBER);
}

















// The delicious_snacks module is trying to present an external
// interface (the `fruit` and `veggie` constants) that is different than
// its internal structure (the `fruits` and `veggies` modules and
// associated constants). It's almost there except for one keyword missing for
// each constant.
