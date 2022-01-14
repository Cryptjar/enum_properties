// Requires the "lazy_static" feature

use enum_properties::enum_properties_lazy;

pub struct FruitProperties {
    pub name:           &'static str,
    pub description:    &'static str,
    pub weight:         f32,
}

enum_properties_lazy! {
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum Fruit: FruitProperties {
        Apple {
            name: "apple",
            description: "Keeps the doctor away.",
            weight: 0.1,
        },
        Orange {
            name: "orange",
            description: "Round and refreshing.",
            weight: 0.13,
        },
        Banana {
            name: "banana",
            description: "Elongated and yellow.",
            weight: 0.12,
        },
    }
}

fn main() {
    println!("An {} weighs about {} kg.",
        Fruit::Apple.name,
        Fruit::Apple.weight
    );
}
