use std::collections::HashMap;

/// Represents a bond with its duration and yield
struct Bond {
    duration: u32,
    yield_rate: f64,
}

/// Represents a trading position
struct Position {
    bond: Bond,
    is_long: bool,
    amount: f64,
}

/// Simulates a yield curve trading strategy
struct YieldCurveTrader {
    positions: Vec<Position>,
    initial_curve: HashMap<u32, f64>,
    final_curve: HashMap<u32, f64>,
}

impl YieldCurveTrader {
    /// Creates a new YieldCurveTrader with the given initial and final yield curves
    fn new(initial_curve: HashMap<u32, f64>, final_curve: HashMap<u32, f64>) -> Self {
        YieldCurveTrader {
            positions: Vec::new(),
            initial_curve,
            final_curve,
        }
    }

    /// Adds a new position to the trading strategy
    fn add_position(&mut self, duration: u32, is_long: bool, amount: f64) {
        let yield_rate = *self.initial_curve.get(&duration).unwrap_or(&0.0);
        let bond = Bond { duration, yield_rate };
        let position = Position { bond, is_long, amount };
        self.positions.push(position);
    }

    /// Calculates the profit or loss for a single position
    fn calculate_position_pl(&self, position: &Position) -> f64 {
        let initial_yield = position.bond.yield_rate;
        let final_yield = *self.final_curve.get(&position.bond.duration).unwrap_or(&initial_yield);
        let yield_change = final_yield - initial_yield;
        
        // Simplified P&L calculation. In reality, this would be more complex.
        let pl = yield_change * position.amount * position.bond.duration as f64;
        if position.is_long { pl } else { -pl }
    }

    /// Calculates the total profit or loss for all positions
    fn calculate_total_pl(&self) -> f64 {
        self.positions.iter().map(|p| self.calculate_position_pl(p)).sum()
    }

    /// Prints a summary of the trading strategy and its outcome
    fn print_summary(&self) {
        println!("Yield Curve Trading Strategy Summary:");
        println!("Initial Yield Curve:");
        for (duration, yield_rate) in &self.initial_curve {
            println!("  {}-year: {:.2}%", duration, yield_rate * 100.0);
        }
        println!("Final Yield Curve:");
        for (duration, yield_rate) in &self.final_curve {
            println!("  {}-year: {:.2}%", duration, yield_rate * 100.0);
        }
        println!("Positions:");
        for position in &self.positions {
            let position_type = if position.is_long { "Long" } else { "Short" };
            println!("  {} {}-year bonds, amount: ${:.2}", position_type, position.bond.duration, position.amount);
        }
        println!("Total P&L: ${:.2}", self.calculate_total_pl());
    }
}

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