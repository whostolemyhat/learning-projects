// Make a programme that allows the user to input either the radius, diameter, or area of the circle.
// The programme should then calculate the other 2 based on the input.

use std::io;


#[derive(Debug)]
struct Circle {
  radius: f64,
  diameter: f64,
  area: f64
}

impl Circle {
  fn radius(diameter: f64) -> f64 {
    diameter / 2f64
  }

  fn area(radius: f64) -> f64 {
    std::f64::consts::PI * (radius * radius)
  }

  fn diameter_from_area(area: f64) -> f64 {
    // sqrt(area/pi) = d
    (area/std::f64::consts::PI).sqrt()
  }

  fn new(radius: f64, diameter: f64, area: f64) -> Circle {
    Circle {
      radius: radius,
      diameter: diameter,
      area: area
    }
  }

  fn new_with_area(area: f64) -> Circle {
    let diameter = diameter_from_area(area);
    let radius = radius(diameter);

    Circle {
      radius: radius,
      diameter: diameter,
      area: area
    }
  }

  fn new_with_radius(radius: f64) -> Circle {
    Circle {
      radius: radius,
      diameter: 2.0 * radius,
      area: area(radius)
    }
  }

  fn new_with_diameter(diameter: f64) -> Circle {
    let radius = radius(diameter);

    Circle {
      diameter: diameter,
      radius: radius,
      area: area(radius)
    }
  }
}

fn main() {
  let mut input = String::new();

  println!("Enter radius:");
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

  let option: f64 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => panic!("That's no number!")
  };

  let circle = Circle::new_with_area(option);
  println!("{:?}", circle);
}
