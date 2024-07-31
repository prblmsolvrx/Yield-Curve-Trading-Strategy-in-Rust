# Yield Curve Trading Strategy Simulator

## Overview

This Rust program simulates a basic Yield Curve Trading Strategy. It allows users to model scenarios where a bond trader believes the yield curve will change in a certain way and calculates potential profits or losses based on initial and final yield curves.

## Features

- Model bonds with different durations and yield rates
- Create long and short positions
- Simulate yield curve changes
- Calculate profit/loss for individual positions and overall strategy
- Provide a summary of the trading strategy and its outcome

## Structure

The program consists of several key components:

1. `Bond`: Represents a bond with its duration and yield rate.
2. `Position`: Represents a trading position, including the bond, whether it's long or short, and the amount.
3. `YieldCurveTrader`: Manages the overall trading strategy, including positions and yield curves.

## Usage

To use this simulator:

1. Set up the initial and final yield curves using `HashMap<u32, f64>`.
2. Create a new `YieldCurveTrader` instance with these yield curves.
3. Add positions using the `add_position` method.
4. Call the `print_summary` method to see the results.

## Example

```rust
fn main() {
    // Set up the initial and final yield curves
    let mut initial_curve = HashMap::new();
    initial_curve.insert(2, 0.0150);
    initial_curve.insert(10, 0.0250);

    let mut final_curve = HashMap::new();
    final_curve.insert(2, 0.0180);
    final_curve.insert(10, 0.0230);

    // Create a new YieldCurveTrader
    let mut trader = YieldCurveTrader::new(initial_curve, final_curve);

    // Add positions
    trader.add_position(2, false, 1_000_000.0);  // Short 2-year bonds
    trader.add_position(10, true, 1_000_000.0);  // Long 10-year bonds

    // Print the summary
    trader.print_summary();
}
```

This example models a scenario where:
- The initial 2-year yield is 1.50% and the 10-year yield is 2.50%
- The final 2-year yield is 1.80% and the 10-year yield is 2.30%
- The trader takes a short position on 2-year bonds and a long position on 10-year bonds

## Key Methods

### `YieldCurveTrader::new(initial_curve: HashMap<u32, f64>, final_curve: HashMap<u32, f64>) -> Self`

Creates a new `YieldCurveTrader` with the given initial and final yield curves.

### `YieldCurveTrader::add_position(&mut self, duration: u32, is_long: bool, amount: f64)`

Adds a new position to the trading strategy. Parameters:
- `duration`: The duration of the bond in years
- `is_long`: `true` for a long position, `false` for a short position
- `amount`: The amount of the position in currency units

### `YieldCurveTrader::calculate_total_pl(&self) -> f64`

Calculates the total profit or loss for all positions.

### `YieldCurveTrader::print_summary(&self)`

Prints a summary of the trading strategy and its outcome, including initial and final yield curves, positions, and total profit/loss.

## Limitations and Future Improvements

1. This is a simplified model. Real-world yield curve trading strategies involve more complex calculations, including bond pricing, duration, convexity, and transaction costs.
2. The profit/loss calculation is basic and doesn't account for many factors that would affect real trades.
3. Error handling is minimal. In a production environment, more robust error handling would be necessary.
4. The program currently only supports two points on the yield curve. It could be expanded to support a full yield curve.

## Contributing

Contributions to improve and expand this yield curve trading strategy simulator are welcome. Please feel free to submit issues or pull requests.


## Disclaimer

This program is for educational and illustrative purposes only. It should not be used for actual trading decisions. Always consult with a qualified financial advisor before making investment decisions.