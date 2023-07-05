use std::collections::HashMap;

fn increment_point(dir: char, current_point: &mut (i32, i32, i32)) {
    match dir {
        '^' => {
            current_point.1 += 1;
        }
        '>' => {
            current_point.0 += 1;
        }
        '<' => {
            current_point.0 -= 1;
        }
        'v' => {
            current_point.1 -= 1;
        }
        _ => {}
    }
}
// And this one is truly a piece of art by me
struct App {
    input: String,
    house_map: HashMap<String, i32>,
}

impl App {
    fn new(sequence: &str) -> App {
        App {
            input: String::from(sequence),
            house_map: HashMap::new(),
        }
    }

    fn update_or_insert_point(&mut self, current_point: (i32, i32, i32)) {
        let house_key = get_house_key(current_point.0, current_point.1);
        if let Some(value) = self.house_map.get(house_key.as_str()) {
            self.house_map.insert(house_key, *value + 1);
        } else {
            self.house_map.insert(house_key, 1);
        }
    }

    fn get_house_delivery_map_for_santa_only(&mut self) -> usize {
        // INFO: Consider santa delivered at first house
        let mut current_point = (0, 0, 1);
        self.house_map.insert(
            get_house_key(current_point.0, current_point.1),
            current_point.2,
        );
        for dir in self.input.clone().chars() {
            increment_point(dir, &mut current_point);
            self.update_or_insert_point(current_point.clone());
        }

        self.house_map.keys().len()
    }

    fn get_house_deliveries_for_both_santa_and_robot(&mut self) -> usize {
        let mut santa_point = (0, 0, 1);
        let mut robo_point = (0, 0, 0);
        let mut is_santa_turn = true;
        self.house_map
            .insert(get_house_key(santa_point.0, santa_point.1), santa_point.2);
        for dir in self.input.clone().chars() {
            if is_santa_turn {
                increment_point(dir, &mut santa_point);
                self.update_or_insert_point(santa_point.clone());
                is_santa_turn = false;
                continue;
            }
            increment_point(dir, &mut robo_point);
            self.update_or_insert_point(robo_point.clone());
            is_santa_turn = true;
        }

        self.house_map.keys().len()
    }
}

fn get_house_key(x: i32, y: i32) -> String {
    format!("({},{})", x, y)
}

fn main() {
    let mut app = App::new(include_str!("../data/day3.txt"));
    println!("{}", app.get_house_delivery_map_for_santa_only());
    println!(
        "{}",
        App::new(include_str!("../data/day3.txt")).get_house_deliveries_for_both_santa_and_robot()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_for_santa_only() {
        assert_eq!(App::new(">").get_house_delivery_map_for_santa_only(), 2);
        assert_eq!(App::new("^>v<").get_house_delivery_map_for_santa_only(), 4);
        assert_eq!(
            App::new("^v^v^v^v^v").get_house_delivery_map_for_santa_only(),
            2
        );
    }

    #[test]
    fn test_for_both_santa_and_robot() {
        assert_eq!(
            App::new("^v").get_house_deliveries_for_both_santa_and_robot(),
            3
        );
        assert_eq!(
            App::new("^>v<").get_house_deliveries_for_both_santa_and_robot(),
            3
        );
        assert_eq!(
            App::new("^v^v^v^v^v").get_house_deliveries_for_both_santa_and_robot(),
            11
        );
    }
}
