/*!
# cuda-platonic

Platonic Forms — ideal templates that agents measure themselves against.

In Plato's cave, the forms are perfect ideals that cast shadows on the wall.
Every real thing is an imperfect instantiation of its form.

Same for agents. Every agent has a *form* — an ideal type describing what
perfect performance looks like. The agent measures its deviation from the form
and evolves toward it. The form itself evolves as the fleet learns.

The form is not a target. It's a compass.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Platonic Form — ideal specification for an agent type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dimensions: Vec<Dimension>,
    pub archetype: Archetype,
    pub evolved_from: Option<String>,
    pub generation: u32,
    pub fitness_history: Vec<f64>,
}

/// A single measurable dimension of a form
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub ideal: f64,         // perfect value [0,1]
    pub tolerance: f64,     // acceptable deviation
    pub weight: f64,        // importance [0,1]
    pub direction: Direction, // higher/lower is better
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Higher, // closer to 1.0 is better
    Lower,  // closer to 0.0 is better
    Exact,  // exact match to ideal
}

/// Agent archetypes — the broad categories
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Archetype {
    Scout,       // fast, cheap, expendable
    Messenger,   // reliable communication
    Navigator,   // pathfinding, planning
    Captain,     // command, strategy
    Artisan,     // creative, generative
    Sentinel,    // monitoring, alerting
    Scholar,     // research, analysis
    Diplomat,    // negotiation, trust-building
}

impl Archetype {
    pub fn default_ideal(archetype: Archetype) -> Vec<Dimension> {
        match archetype {
            Archetype::Scout => vec![
                dim("speed", 0.9, 0.2, 0.3, Direction::Higher),
                dim("stealth", 0.8, 0.3, 0.2, Direction::Higher),
                dim("endurance", 0.5, 0.3, 0.1, Direction::Higher),
                dim("cost", 0.2, 0.2, 0.4, Direction::Lower),
            ],
            Archetype::Messenger => vec![
                dim("reliability", 0.95, 0.05, 0.4, Direction::Higher),
                dim("throughput", 0.8, 0.2, 0.2, Direction::Higher),
                dim("latency", 0.1, 0.1, 0.3, Direction::Lower),
                dim("trust", 0.9, 0.1, 0.3, Direction::Higher),
            ],
            Archetype::Navigator => vec![
                dim("accuracy", 0.9, 0.1, 0.3, Direction::Higher),
                dim("adaptability", 0.8, 0.2, 0.3, Direction::Higher),
                dim("foresight", 0.7, 0.2, 0.2, Direction::Higher),
                dim("cost", 0.3, 0.2, 0.2, Direction::Lower),
            ],
            Archetype::Captain => vec![
                dim("strategy", 0.95, 0.05, 0.3, Direction::Higher),
                dim("trust", 0.9, 0.1, 0.2, Direction::Higher),
                dim("decisiveness", 0.85, 0.1, 0.2, Direction::Higher),
                dim("fleet_health", 0.8, 0.2, 0.3, Direction::Higher),
            ],
            Archetype::Artisan => vec![
                dim("creativity", 0.9, 0.2, 0.3, Direction::Higher),
                dim("quality", 0.85, 0.1, 0.3, Direction::Higher),
                dim("novelty", 0.8, 0.2, 0.2, Direction::Higher),
                dim("consistency", 0.7, 0.2, 0.1, Direction::Higher),
            ],
            Archetype::Sentinel => vec![
                dim("vigilance", 0.95, 0.05, 0.3, Direction::Higher),
                dim("precision", 0.9, 0.1, 0.3, Direction::Higher),
                dim("endurance", 0.9, 0.1, 0.3, Direction::Higher),
                dim("false_positives", 0.05, 0.1, 0.2, Direction::Lower),
            ],
            Archetype::Scholar => vec![
                dim("depth", 0.9, 0.1, 0.3, Direction::Higher),
                dim("breadth", 0.7, 0.2, 0.2, Direction::Higher),
                dim("synthesis", 0.85, 0.1, 0.3, Direction::Higher),
                dim("speed", 0.5, 0.3, 0.1, Direction::Higher),
            ],
            Archetype::Diplomat => vec![
                dim("empathy", 0.9, 0.1, 0.3, Direction::Higher),
                dim("negotiation", 0.9, 0.1, 0.3, Direction::Higher),
                dim("trust_building", 0.9, 0.1, 0.3, Direction::Higher),
                dim("patience", 0.8, 0.2, 0.2, Direction::Higher),
            ],
        }
    }
}

/// Measurement of an agent against a form
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub agent_id: String,
    pub form_id: String,
    pub scores: Vec<DimensionScore>,
    pub overall: f64,         // weighted average deviation
    pub distance: f64,        // Euclidean distance from ideal
    pub confidence: f64,      // confidence in measurement
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DimensionScore {
    pub name: String,
    pub actual: f64,
    pub ideal: f64,
    pub deviation: f64,
    pub within_tolerance: bool,
}

/// The form library — manages all platonic forms
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormLibrary {
    pub forms: HashMap<String, Form>,
    pub measurements: Vec<Measurement>,
}

impl FormLibrary {
    pub fn new() -> Self { FormLibrary { forms: HashMap::new(), measurements: vec![] } }

    pub fn add_form(&mut self, form: Form) { self.forms.insert(form.id.clone(), form); }

    /// Create form from archetype with defaults
    pub fn create_from_archetype(id: &str, name: &str, archetype: Archetype) -> Form {
        Form { id: id.to_string(), name: name.to_string(), description: format!("{:?} archetype", archetype), dimensions: Archetype::default_ideal(archetype), archetype, evolved_from: None, generation: 0, fitness_history: vec![] }
    }

    /// Measure an agent against a form
    pub fn measure(&self, form_id: &str, agent_id: &str, actuals: &HashMap<String, f64>) -> Option<Measurement> {
        let form = self.forms.get(form_id)?;
        let mut scores = vec![];
        let mut weighted_sum = 0.0;
        let mut weight_total = 0.0;
        let mut sq_dist = 0.0;

        for dim in &form.dimensions {
            let actual = actuals.get(&dim.name).copied().unwrap_or(0.0);
            let deviation = (actual - dim.ideal).abs();
            let within = deviation <= dim.tolerance;

            scores.push(DimensionScore { name: dim.name.clone(), actual, ideal: dim.ideal, deviation, within_tolerance: within });

            weighted_sum += deviation * dim.weight;
            weight_total += dim.weight;
            sq_dist += (deviation / dim.tolerance.max(0.001)).powi(2);
        }

        let overall = if weight_total > 0.0 { weighted_sum / weight_total } else { 1.0 };
        let distance = sq_dist.sqrt() / (form.dimensions.len() as f64).sqrt();

        Some(Measurement { agent_id: agent_id.to_string(), form_id: form_id.to_string(), scores, overall: 1.0 - overall.clamp(0.0, 1.0), distance, confidence: 1.0, timestamp: now() })
    }

    /// Evolve a form based on fleet measurements (move ideals toward fleet median)
    pub fn evolve(&mut self, form_id: &str, measurements: &[Measurement]) -> Option<Form> {
        let form = self.forms.get_mut(form_id)?;
        if measurements.is_empty() || form.dimensions.is_empty() { return None; }

        for dim in &mut form.dimensions {
            let mut vals: Vec<f64> = measurements.iter()
                .filter_map(|m| m.scores.iter().find(|s| s.name == dim.name).map(|s| s.actual))
                .collect();
            if vals.is_empty() { continue; }
            vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = vals[vals.len() / 2];
            // Shift ideal 10% toward fleet median
            dim.ideal = dim.ideal * 0.9 + median * 0.1;
        }

        form.generation += 1;
        Some(form.clone())
    }

    /// Distance between two forms
    pub fn form_distance(&self, id_a: &str, id_b: &str) -> Option<f64> {
        let a = self.forms.get(id_a)?;
        let b = self.forms.get(id_b)?;
        let mut sq = 0.0;
        let mut count = 0;
        for da in &a.dimensions {
            if let Some(db) = b.dimensions.iter().find(|d| d.name == da.name) {
                sq += (da.ideal - db.ideal).powi(2);
                count += 1;
            }
        }
        if count == 0 { return None; }
        Some(sq.sqrt() / count as f64)
    }
}

fn dim(name: &str, ideal: f64, tolerance: f64, weight: f64, dir: Direction) -> Dimension {
    Dimension { name: name.to_string(), ideal, tolerance, weight, direction: dir }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_form() {
        let form = FormLibrary::create_from_archetype("scout-1", "Scout", Archetype::Scout);
        assert_eq!(form.archetype, Archetype::Scout);
        assert!(!form.dimensions.is_empty());
    }

    #[test]
    fn test_measure_perfect() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("s", "S", Archetype::Scout));
        let mut actuals = HashMap::new();
        for dim in &lib.forms["s"].dimensions { actuals.insert(dim.name.clone(), dim.ideal); }
        let m = lib.measure("s", "ant1", &actuals).unwrap();
        assert!(m.overall > 0.99);
        assert!(m.distance < 0.01);
    }

    #[test]
    fn test_measure_deviated() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("s", "S", Archetype::Scout));
        let actuals: HashMap<String, f64> = vec![("speed".into(), 0.5)].into_iter().collect();
        let m = lib.measure("s", "ant1", &actuals).unwrap();
        assert!(m.overall < 1.0);
    }

    #[test]
    fn test_tolerance() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("s", "S", Archetype::Scout));
        let dim = &lib.forms["s"].dimensions[0]; // speed, ideal=0.9, tolerance=0.2
        let mut actuals = HashMap::new();
        actuals.insert("speed".into(), dim.ideal + dim.tolerance - 0.01); // just within
        let m = lib.measure("s", "ant1", &actuals).unwrap();
        assert!(m.scores[0].within_tolerance);
    }

    #[test]
    fn test_evolve() {
        let mut lib = FormLibrary::new();
        let form = FormLibrary::create_from_archetype("s", "S", Archetype::Scout);
        let old_ideal = form.dimensions[0].ideal;
        lib.add_form(form);
        let mut actuals = HashMap::new();
        actuals.insert("speed".into(), 0.5);
        let m = lib.measure("s", "a", &actuals).unwrap();
        lib.evolve("s", &[m]);
        assert_eq!(lib.forms["s"].generation, 1);
        // Ideal should have shifted toward 0.5
        assert!((lib.forms["s"].dimensions[0].ideal - old_ideal).abs() > 0.001);
    }

    #[test]
    fn test_form_distance() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("a", "A", Archetype::Scout));
        lib.add_form(FormLibrary::create_from_archetype("b", "B", Archetype::Messenger));
        let dist = lib.form_distance("a", "b");
        assert!(dist.is_some());
        assert!(dist.unwrap() > 0.0);
    }

    #[test]
    fn test_same_form_distance() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("a", "A", Archetype::Scout));
        let dist = lib.form_distance("a", "a");
        assert_eq!(dist, Some(0.0));
    }

    #[test]
    fn test_missing_dimension() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("s", "S", Archetype::Scout));
        let actuals = HashMap::new(); // empty
        let m = lib.measure("s", "a", &actuals).unwrap();
        // All dimensions default to 0.0, which is deviated
        assert!(m.overall < 1.0);
    }

    #[test]
    fn test_all_archetypes_have_dimensions() {
        for arch in [Archetype::Scout, Archetype::Messenger, Archetype::Navigator, Archetype::Captain, Archetype::Artisan, Archetype::Sentinel, Archetype::Scholar, Archetype::Diplomat] {
            let dims = Archetype::default_ideal(arch);
            assert!(!dims.is_empty(), "{:?} has no dimensions", arch);
        }
    }

    #[test]
    fn test_measurement_agent_id() {
        let mut lib = FormLibrary::new();
        lib.add_form(FormLibrary::create_from_archetype("s", "S", Archetype::Scout));
        let m = lib.measure("s", "my-agent", &HashMap::new()).unwrap();
        assert_eq!(m.agent_id, "my-agent");
    }
}
