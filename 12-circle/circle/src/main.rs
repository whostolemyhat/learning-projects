// Make a programme that allows the user to input either the radius, diameter, or area of the circle.
// The programme should then calculate the other 2 based on the input.

use std::io;

#[derive(Debug)]
pub struct Circle {
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
    // Self:: or Circle::
    // Self = static type
    // Circle = explicit type
    let diameter = Self::diameter_from_area(area);
    let radius = Self::radius(diameter);

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
      area: Self::area(radius)
    }
  }

  fn new_with_diameter(diameter: f64) -> Circle {
    let radius = Self::radius(diameter);

    Circle {
      diameter: diameter,
      radius: radius,
      area: Self::area(radius)
    }
  }
}

fn main() {
  let mut input = String::new();

  println!("Pick an option:");
  println!("1. Area");
  println!("2. Radius");
  println!("3. Diameter");

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

  let option: f64 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => panic!("That's no number!")
  };

  match option {
    1f64 => println!("Enter area:"),
    2f64 => println!("Enter radius:"),
    3f64 => println!("Enter diameter:"),
    _ => panic!("Pick one of the options.")
  };

  let mut size_string = String::new();
  io::stdin()
    .read_line(&mut size_string)
    .expect("Failed to read input");

  match size_string.trim().parse() {
    Ok(num) => {
      let circle = match option {
        1f64 => Circle::new_with_area(num),
        2f64 => Circle::new_with_radius(num),
        3f64 => Circle::new_with_diameter(num),
        _ => Circle::new(0.0, 0.0, 0.0)
      };

      println!("{:?}", circle);
    }
    Err(_) => panic!("That's no number")
  };
}
