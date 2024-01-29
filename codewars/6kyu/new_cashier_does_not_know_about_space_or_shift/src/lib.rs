

fn get_order(input: String ) -> String {

    let menu = vec![
        MenuItem::new("Burger".to_string(), 1),
        MenuItem::new("Fries".to_string(), 2),
        MenuItem::new("Chicken".to_string(), 3),
        MenuItem::new("Pizza".to_string(), 4),
        MenuItem::new("Sandwich".to_string(), 5),
        MenuItem::new("Onionrings".to_string(), 6),
        MenuItem::new("Milkshake".to_string(), 7),
        MenuItem::new("Coke".to_string(), 8)
    ];

    menu.iter()
            .filter_map(|item| {
                let count = input.matches(&item.name.to_lowercase()).count();
                if count > 0 {
                    Some((item.name.to_string() + " ").repeat(count))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("")
            .trim_end()
            .to_string()
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct MenuItem {
    name: String,
    sequence: u32,
}

impl MenuItem {
    pub fn new(name: String, sequence: u32) -> Self {
        MenuItem {
            name,
            sequence,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!( get_order( "milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza".to_string() ),
                    "Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke".to_string() );
    }

    #[test]
    fn test_2() {
        assert_eq!( get_order( "pizzachickenfriesburgercokemilkshakefriessandwich".to_string() ),
                    "Burger Fries Fries Chicken Pizza Sandwich Milkshake Coke".to_string() );
    }
}