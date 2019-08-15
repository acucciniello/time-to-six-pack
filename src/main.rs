#[macro_use] extern crate text_io;

fn validate_inputs(weight: f32, body_fat_percentage: f32, goal_body_fat_percentage: f32, rate_of_weekly_weight_loss: f32) -> bool {
    // let mut output: String = "";
    let mut are_inputs_valid: bool = true;
    if weight <= 0.0 {
      are_inputs_valid = false;
      // output = "Weight must be a decimal value greater than 0.0";
    } else if body_fat_percentage <= 0.0 && body_fat_percentage >= 100.0 {
      are_inputs_valid = false;
      // output = "Body fat percentage must be greater than 0.0 and must be shown as a decimal (i.e. 11.5% bodyfat is 0.115)";
    } else if goal_body_fat_percentage <= 0.0 && goal_body_fat_percentage >= 100.0 {
      are_inputs_valid = false;
      // output = "Goal Body fat percentage must be greater than 0.0 and must be shown as a decimal (i.e. 11.5% bodyfat is 0.115)";
    } else if rate_of_weekly_weight_loss > 2.0 || rate_of_weekly_weight_loss <= 0.0 {
      are_inputs_valid = false;
      // output = "Rate of weekly weight loss needs to be at greater than 0.0 pounds, and no greater than 2.0 pounds.  Also it must be a decimal.";
    }
  // println!(output);
  return are_inputs_valid;
}

fn calculate_current_lean_body_mass(weight: f32, body_fat_percentage: f32) -> f32{
  let current_lean_body_mass: f32 = weight * (1.0 - body_fat_percentage);
  return current_lean_body_mass;
}

fn calculate_goal_lean_body_mass(current_lean_body_mass: f32, body_fat_percentage: f32) -> f32 {
  let mut lost_lean_body_fat_percentage: f32 = 0.97;
  if body_fat_percentage < 0.12 && body_fat_percentage > 0.10 {
    lost_lean_body_fat_percentage = 0.98;
  } else if body_fat_percentage < 0.10 {
    lost_lean_body_fat_percentage = 0.99;
  } 
  let goal_lean_body_mass:f32 = current_lean_body_mass * lost_lean_body_fat_percentage;
  return goal_lean_body_mass;
}

fn calculate_goal_weight(goal_lean_body_mass: f32, goal_body_fat_percentage: f32) -> f32 {
  let goal_weight: f32 = goal_lean_body_mass/(1.0 - goal_body_fat_percentage);
  return goal_weight;
}

fn calculate_time_to_six_pack(weight: f32, goal_weight: f32, rate_of_weekly_weight_loss: f32) -> f32 {
  let time_to_six_pack: f32 = (weight - goal_weight)/rate_of_weekly_weight_loss;
  return time_to_six_pack;
}

fn main() {
    println!("What is your current body weight?");
    let weight: f32 = read!();
    println!("What is your current body fat percentage?");
    let body_fat_percentage:f32 = read!();
    println!("What is your goal body fat percentage?");
    let goal_body_fat_percentage:f32 = read!();
    println!("What is your desired rate of weight loos per week? Answers can be between 0.1 pounds to 2 pounds per week.");
    let rate_of_weekly_weight_loss:f32 = read!();

    let are_inputs_valid:bool = validate_inputs(weight, body_fat_percentage, goal_body_fat_percentage, rate_of_weekly_weight_loss);
    if are_inputs_valid {
      let current_lean_body_mass:f32 = calculate_current_lean_body_mass(weight, body_fat_percentage);
      let goal_lean_body_mass: f32 = calculate_goal_lean_body_mass(current_lean_body_mass, body_fat_percentage);
      let goal_weight: f32 = calculate_goal_weight(goal_lean_body_mass, goal_body_fat_percentage);
      let time_to_six_pack: f32 = calculate_time_to_six_pack(weight, goal_weight, rate_of_weekly_weight_loss);
      println!("{} weeks till you have a 6 pack if you lost {} pounds per week.", time_to_six_pack, rate_of_weekly_weight_loss);
    }   
}

