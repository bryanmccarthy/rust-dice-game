use std::io;
use rand::distributions::{Distribution, Uniform};

pub fn game_loop() {

  let mut tokens:i32 = 5;

  loop {
    println!("\n*** Dice Game ***");

    println!("-- Token(s) --");
    println!("{}", tokens);

    println!("-- Menu --");
    menu();
    let choice = menu_choice();
    match choice {
      1 => higher_lower(&mut tokens),
      2 => snake_eyes(&mut tokens),
      3 => break,
      _ => not_an_option(),
    }
  }
}

fn menu() {
  println!("(1) Higher Lower (Win 1 Token, Lose 1 Token)");
  println!("(2) Snake Eyes (Win 3 Tokens, Lose 1 Token)");
  println!("(3) Exit");
}

fn not_an_option() {
  println!("Not an option");
}

fn higher_lower(tokens: &mut i32) {
  loop {
    if *tokens < 1 {
      println!("\nNot enough tokens");
      break;
    }

    let number = dice_roll();
    println!("\n* Higher or Lower *");
    println!("Number: {}", number);

    println!("Higer or Lower? (h/l)");
    let ans = h_or_l();

    let rolled_number = dice_roll();

    if ans == true {
      if rolled_number > number {
        println!("You rolled {}, You win 1 Token!", rolled_number);
        *tokens += 1;
      } else {
      println!("You rolled {}, You Lose 1 Token", rolled_number);
      *tokens -= 1;
      }
    } else {
      if rolled_number < number {
        println!("You rolled {}, You win 1 Token!", rolled_number);
        *tokens += 1;
      } else {
        println!("You rolled {}, You Lose 1 Token", rolled_number);
        *tokens -= 1;
      }
    }

    println!("\nplay again? (y/n)");
    let ans = y_or_n();
    if ans == false {
      println!("exiting...");
      break;
    }
  }
}

fn menu_choice() -> i8 {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let int_input: i8 = input.trim().parse().expect("invalid input");
  return int_input;
}

fn dice_roll() -> i8 {
  let mut rng = rand::thread_rng();
  let dice_range = Uniform::from(1..7);

  let roll = dice_range.sample(&mut rng);
  return roll;
}

fn h_or_l() -> bool {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  return input.to_ascii_lowercase().starts_with("h");
}

fn y_or_n() -> bool {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  return input.to_ascii_lowercase().starts_with("y");
}

fn snake_eyes(tokens: &mut i32) {
  loop {
    if *tokens < 1 {
      println!("\nNot enough tokens");
      break;
    }

    println!("\n* Snake Eyes -- Roll two 1's in a row to win *");
    println!("Rolling...");

    let mut rolled_number = dice_roll();
    println!("You rolled a {}", rolled_number);

    if rolled_number == 1 {
      println!("Rolling again...");
      rolled_number = dice_roll();
      println!("You rolled a {}", rolled_number);
      if rolled_number == 1 {
        println!("You win 3 Tokens!");
        *tokens += 3;
      } else {
        println!("You Lose 1 Token");
        *tokens -= 1;
      }

      println!("\nplay again? (y/n)");
      let ans = y_or_n();
      if ans == false {
        println!("exiting...");
        break;
      }
    }
  }
}