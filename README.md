# time-to-six-pack 

A Rust CLI to calculate how many weeks until you have a six pack when on a diet.  


## Requirements

[Rust v1.35.0](https://www.rust-lang.org/tools/install)

## Getting Started

1. Clone the repository:

```
$ git clone https://github.com/acucciniello/time-to-six-pack.git
```

2. Enter the directory

```
$ cd time-to-six-pack
```

3. Run the CLI App:

```
$ cargo run
```

## Usage

It will prompt you for the following information:

1. Your Current Weight in Pounds (Represented as a decimal)
2. Your Current BodyFat in decimal form (i.e. 11.5% should be inputted as 0.115)
3. Your Goal BodyFat in decimal form (i.e. 11.5% should be inputted as 0.115)
4. Your Desired Weekly Weight Loss (in decimal form from a 0.1 to 2.0 pounds per week.)

Then it will return how many weeks till your have a 6 pack:
```
5.8223877 weeks till you have a 6 pack if you lost 1 pounds per week.
```

## References
1. This is inspired by VirtuvianPhysique's (YouTube Video)[https://www.youtube.com/watch?v=j4zOuCYYCcs] where he did this calculation
2. The scale I use to measure my weight and body fat are (here)[https://amzn.to/2Z2Ry5z]

## License

MIT
