#![forbid(unsafe_code)]

//! Symbiotic relationships between ternary agents.
//!
//! Agents sharing a room develop relationships over time: mutual benefit
//! (mutualism), one-sided benefit (commensalism), or exploitation (parasitism).
//! This crate models these patterns with `SymbiontPair`, `ParasiticPair`,
//! `CommensalPair`, plus a `SymbiosisDetector` that discovers beneficial
//! partnerships and a `SymbiosisEvolver` that co-evolves symbiotic pairs.

use std::collections::HashMap;

// ── Agent ──────────────────────────────────────────────────────────────────

/// A ternary agent with a fitness score and trait vector.
#[derive(Debug, Clone, PartialEq)]
pub struct Agent {
    pub id: String,
    /// Trait vector using ternary values: -1.0, 0.0, +1.0.
    pub traits: Vec<f64>,
    /// Current fitness score (higher is better).
    pub fitness: f64,
}

impl Agent {
    pub fn new(id: &str, traits: Vec<f64>, fitness: f64) -> Self {
        Self {
            id: id.to_string(),
            traits,
            fitness,
        }
    }

    /// Create an agent with default fitness.
    pub fn with_traits(id: &str, traits: Vec<f64>) -> Self {
        Self::new(id, traits, 0.0)
    }

    /// How many traits this agent has.
    pub fn trait_count(&self) -> usize {
        self.traits.len()
    }
}

// ── SymbiosisType ──────────────────────────────────────────────────────────

/// The kind of symbiotic relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbiosisType {
    /// Both agents benefit.
    Mutualism,
    /// One benefits, one is harmed.
    Parasitism,
    /// One benefits, one is unaffected.
    Commensalism,
}

// ── Compatibility ──────────────────────────────────────────────────────────

/// Result of computing compatibility between two agents.
#[derive(Debug, Clone, PartialEq)]
pub struct Compatibility {
    /// Cosine-like similarity score in [-1.0, 1.0].
    pub score: f64,
    /// Which symbiosis type best describes this pair.
    pub relationship: SymbiosisType,
}

impl Compatibility {
    pub fn new(score: f64, relationship: SymbiosisType) -> Self {
        Self { score, relationship }
    }
}

/// Compute compatibility between two agents based on their trait vectors.
/// Uses dot product normalized by vector length.
pub fn compute_compatibility(a: &Agent, b: &Agent) -> Compatibility {
    if a.traits.len() != b.traits.len() || a.traits.is_empty() {
        return Compatibility::new(0.0, SymbiosisType::Commensalism);
    }

    let dot: f64 = a.traits.iter().zip(&b.traits).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.traits.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.traits.iter().map(|x| x * x).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return Compatibility::new(0.0, SymbiosisType::Commensalism);
    }

    let score = dot / (norm_a * norm_b);

    let relationship = if score > 0.3 {
        SymbiosisType::Mutualism
    } else if score < -0.3 {
        SymbiosisType::Parasitism
    } else {
        SymbiosisType::Commensalism
    };

    Compatibility::new(score, relationship)
}

// ── SymbiontPair ───────────────────────────────────────────────────────────

/// Two agents in a mutualistic relationship where both benefit.
#[derive(Debug, Clone)]
pub struct SymbiontPair {
    pub agent_a: Agent,
    pub agent_b: Agent,
    /// How many interaction cycles they've been together.
    pub bond_strength: f64,
}

impl SymbiontPair {
    pub fn new(a: Agent, b: Agent) -> Self {
        Self {
            agent_a: a,
            agent_b: b,
            bond_strength: 1.0,
        }
    }

    /// Perform one interaction cycle. Both agents gain fitness proportional
    /// to their compatibility and bond strength.
    pub fn interact(&mut self) -> f64 {
        let compat = compute_compatibility(&self.agent_a, &self.agent_b);
        let gain = compat.score.max(0.0) * self.bond_strength * 0.1;
        self.agent_a.fitness += gain;
        self.agent_b.fitness += gain;
        self.bond_strength += 0.1;
        gain
    }

    /// Check if the pair is still compatible (positive compatibility).
    pub fn is_healthy(&self) -> bool {
        let compat = compute_compatibility(&self.agent_a, &self.agent_b);
        compat.score > 0.0
    }

    /// Get the combined fitness of both agents.
    pub fn combined_fitness(&self) -> f64 {
        self.agent_a.fitness + self.agent_b.fitness
    }
}

// ── ParasiticPair ──────────────────────────────────────────────────────────

/// Two agents in a parasitic relationship: one benefits at the other's expense.
#[derive(Debug, Clone)]
pub struct ParasiticPair {
    pub parasite: Agent,
    pub host: Agent,
    /// How much fitness the parasite drains per cycle.
    pub drain_rate: f64,
}

impl ParasiticPair {
    pub fn new(parasite: Agent, host: Agent, drain_rate: f64) -> Self {
        Self { parasite, host, drain_rate }
    }

    /// One cycle: parasite gains, host loses.
    pub fn parasitize(&mut self) -> (f64, f64) {
        let gain = self.drain_rate;
        self.parasite.fitness += gain;
        self.host.fitness -= gain;
        (gain, -gain)
    }

    /// Check if the host is still alive (fitness > 0).
    pub fn host_alive(&self) -> bool {
        self.host.fitness > 0.0
    }

    /// How many more cycles before the host runs out of fitness.
    pub fn cycles_remaining(&self) -> u64 {
        if self.drain_rate <= 0.0 {
            return u64::MAX;
        }
        (self.host.fitness / self.drain_rate).max(0.0) as u64
    }
}

// ── CommensalPair ──────────────────────────────────────────────────────────

/// Two agents in a commensal relationship: one benefits, one is unaffected.
#[derive(Debug, Clone)]
pub struct CommensalPair {
    pub benefactor: Agent,
    pub neutral: Agent,
    /// How much the benefactor gains per cycle from proximity.
    pub benefit_rate: f64,
}

impl CommensalPair {
    pub fn new(benefactor: Agent, neutral: Agent, benefit_rate: f64) -> Self {
        Self { benefactor, neutral, benefit_rate }
    }

    /// One cycle: benefactor gains, neutral is unchanged.
    pub fn commensalize(&mut self) -> f64 {
        let gain = self.benefit_rate;
        self.benefactor.fitness += gain;
        gain
    }
}

// ── SymbiosisDetector ──────────────────────────────────────────────────────

/// Discovers potentially beneficial partnerships from a pool of agents.
pub struct SymbiosisDetector {
    /// Minimum compatibility score to consider a partnership.
    pub threshold: f64,
    /// Maximum number of partnerships to return.
    pub max_results: usize,
}

impl SymbiosisDetector {
    pub fn new(threshold: f64, max_results: usize) -> Self {
        Self { threshold, max_results }
    }

    /// Scan all pairs and return the best partnerships sorted by score descending.
    pub fn detect(&self, agents: &[Agent]) -> Vec<Compatibility> {
        let mut results = Vec::new();
        for i in 0..agents.len() {
            for j in (i + 1)..agents.len() {
                let compat = compute_compatibility(&agents[i], &agents[j]);
                if compat.score >= self.threshold {
                    results.push(compat);
                }
            }
        }
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(self.max_results);
        results
    }

    /// Find the single best partner for a given agent.
    pub fn best_partner(&self, agent: &Agent, candidates: &[Agent]) -> Option<(usize, Compatibility)> {
        let mut best: Option<(usize, Compatibility)> = None;
        for (i, candidate) in candidates.iter().enumerate() {
            let compat = compute_compatibility(agent, candidate);
            if compat.score >= self.threshold {
                if best.as_ref().map_or(true, |(_, c)| compat.score > c.score) {
                    best = Some((i, compat));
                }
            }
        }
        best
    }
}

// ── SymbiosisEvolver ───────────────────────────────────────────────────────

/// Co-evolves symbiotic pairs over generations.
pub struct SymbiosisEvolver {
    /// Mutation rate: how much traits can change per generation (0.0 - 1.0).
    pub mutation_rate: f64,
    /// Number of generations to run per evolve call.
    pub generations: u32,
}

impl SymbiosisEvolver {
    pub fn new(mutation_rate: f64, generations: u32) -> Self {
        Self { mutation_rate, generations }
    }

    /// Co-evolve a pair: agents influence each other's traits, then mutate.
    /// Returns the pair after evolution.
    pub fn evolve_pair(&self, pair: &mut SymbiontPair) {
        for _ in 0..self.generations {
            // Cross-influence: each agent shifts traits slightly toward the other
            let len = pair.agent_a.traits.len().min(pair.agent_b.traits.len());
            for i in 0..len {
                let diff = pair.agent_b.traits[i] - pair.agent_a.traits[i];
                let shift = diff * self.mutation_rate * 0.1;
                pair.agent_a.traits[i] += shift;
                pair.agent_b.traits[i] -= shift;
            }

            // Small random-ish mutation: shift a trait by mutation_rate * ±1
            // Since no external deps, we use a deterministic "pseudo-mutation"
            // based on trait index and current value.
            if !pair.agent_a.traits.is_empty() {
                let idx = (pair.agent_a.fitness as usize) % pair.agent_a.traits.len();
                if idx < pair.agent_a.traits.len() {
                    pair.agent_a.traits[idx] += self.mutation_rate * 0.1;
                }
            }
            if !pair.agent_b.traits.is_empty() {
                let idx = (pair.agent_b.fitness as usize) % pair.agent_b.traits.len();
                if idx < pair.agent_b.traits.len() {
                    pair.agent_b.traits[idx] += self.mutation_rate * 0.1;
                }
            }
        }
    }

    /// Evolve a population of agents: pair them, evolve each pair, return sorted by fitness.
    pub fn evolve_population(&self, agents: &mut [Agent]) {
        // Pair adjacent agents
        let mut i = 0;
        while i + 1 < agents.len() {
            let compat = compute_compatibility(&agents[i], &agents[i + 1]);
            if compat.score > 0.0 {
                // Cross-influence
                let len = agents[i].traits.len().min(agents[i + 1].traits.len());
                for j in 0..len {
                    let diff = agents[i + 1].traits[j] - agents[i].traits[j];
                    let shift = diff * self.mutation_rate * 0.05;
                    agents[i].traits[j] += shift;
                    agents[i + 1].traits[j] -= shift;
                }
            }
            i += 2;
        }

        // Update fitness based on trait sum
        for agent in agents.iter_mut() {
            let trait_sum: f64 = agent.traits.iter().sum();
            agent.fitness = trait_sum * 0.5 + agent.fitness * 0.5;
        }
    }
}

// ── RelationshipLog ────────────────────────────────────────────────────────

/// A log of relationship events between agents.
#[derive(Debug, Clone)]
pub struct RelationshipLog {
    pub events: Vec<RelationshipEvent>,
}

/// A single relationship event.
#[derive(Debug, Clone)]
pub struct RelationshipEvent {
    pub agent_a: String,
    pub agent_b: String,
    pub event_type: SymbiosisType,
    pub fitness_delta: f64,
    pub tick: u64,
}

impl RelationshipLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn log(&mut self, a: &str, b: &str, event_type: SymbiosisType, delta: f64, tick: u64) {
        self.events.push(RelationshipEvent {
            agent_a: a.to_string(),
            agent_b: b.to_string(),
            event_type,
            fitness_delta: delta,
            tick,
        });
    }

    /// Get all events involving a given agent.
    pub fn events_for(&self, agent_id: &str) -> Vec<&RelationshipEvent> {
        self.events.iter()
            .filter(|e| e.agent_a == agent_id || e.agent_b == agent_id)
            .collect()
    }

    /// Count events by type.
    pub fn count_by_type(&self) -> HashMap<SymbiosisType, usize> {
        let mut counts = HashMap::new();
        for event in &self.events {
            *counts.entry(event.event_type).or_insert(0) += 1;
        }
        counts
    }

    /// Total number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for RelationshipLog {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let a = Agent::new("a1", vec![1.0, 0.0, -1.0], 5.0);
        assert_eq!(a.id, "a1");
        assert_eq!(a.trait_count(), 3);
        assert_eq!(a.fitness, 5.0);
    }

    #[test]
    fn test_compute_compatibility_mutual() {
        let a = Agent::with_traits("a", vec![1.0, 1.0, 1.0]);
        let b = Agent::with_traits("b", vec![1.0, 1.0, 1.0]);
        let compat = compute_compatibility(&a, &b);
        assert!(compat.score > 0.9);
        assert_eq!(compat.relationship, SymbiosisType::Mutualism);
    }

    #[test]
    fn test_compute_compatibility_parasitic() {
        let a = Agent::with_traits("a", vec![1.0, 1.0, 1.0]);
        let b = Agent::with_traits("b", vec![-1.0, -1.0, -1.0]);
        let compat = compute_compatibility(&a, &b);
        assert!(compat.score < -0.9);
        assert_eq!(compat.relationship, SymbiosisType::Parasitism);
    }

    #[test]
    fn test_compute_compatibility_commensal() {
        let a = Agent::with_traits("a", vec![1.0, 0.0, -1.0]);
        let b = Agent::with_traits("b", vec![0.0, 1.0, 0.0]);
        let compat = compute_compatibility(&a, &b);
        assert_eq!(compat.relationship, SymbiosisType::Commensalism);
    }

    #[test]
    fn test_compute_compatibility_empty() {
        let a = Agent::with_traits("a", vec![]);
        let b = Agent::with_traits("b", vec![]);
        let compat = compute_compatibility(&a, &b);
        assert_eq!(compat.score, 0.0);
    }

    #[test]
    fn test_compute_compatibility_different_lengths() {
        let a = Agent::with_traits("a", vec![1.0, 1.0]);
        let b = Agent::with_traits("b", vec![1.0]);
        let compat = compute_compatibility(&a, &b);
        assert_eq!(compat.score, 0.0);
    }

    #[test]
    fn test_symbiont_pair_interact() {
        let a = Agent::with_traits("a", vec![1.0, 1.0]);
        let b = Agent::with_traits("b", vec![1.0, 1.0]);
        let mut pair = SymbiontPair::new(a, b);
        let gain = pair.interact();
        assert!(gain > 0.0);
        assert!(pair.agent_a.fitness > 0.0);
        assert!(pair.agent_b.fitness > 0.0);
        assert!(pair.bond_strength > 1.0);
    }

    #[test]
    fn test_symbiont_pair_healthy() {
        let a = Agent::with_traits("a", vec![1.0]);
        let b = Agent::with_traits("b", vec![1.0]);
        let pair = SymbiontPair::new(a, b);
        assert!(pair.is_healthy());
    }

    #[test]
    fn test_symbiont_pair_combined_fitness() {
        let a = Agent::new("a", vec![1.0], 3.0);
        let b = Agent::new("b", vec![1.0], 7.0);
        let pair = SymbiontPair::new(a, b);
        assert_eq!(pair.combined_fitness(), 10.0);
    }

    #[test]
    fn test_parasitic_pair() {
        let parasite = Agent::new("p", vec![-1.0], 0.0);
        let host = Agent::new("h", vec![1.0], 10.0);
        let mut pair = ParasiticPair::new(parasite, host, 2.0);
        let (gain, loss) = pair.parasitize();
        assert_eq!(gain, 2.0);
        assert_eq!(loss, -2.0);
        assert_eq!(pair.parasite.fitness, 2.0);
        assert_eq!(pair.host.fitness, 8.0);
    }

    #[test]
    fn test_parasitic_host_alive() {
        let p = Agent::new("p", vec![], 0.0);
        let h = Agent::new("h", vec![], 1.0);
        let pair = ParasiticPair::new(p, h, 0.5);
        assert!(pair.host_alive());
    }

    #[test]
    fn test_parasitic_cycles_remaining() {
        let p = Agent::new("p", vec![], 0.0);
        let h = Agent::new("h", vec![], 10.0);
        let pair = ParasiticPair::new(p, h, 2.0);
        assert_eq!(pair.cycles_remaining(), 5);
    }

    #[test]
    fn test_commensal_pair() {
        let benefactor = Agent::new("b", vec![1.0], 0.0);
        let neutral = Agent::new("n", vec![0.0], 5.0);
        let mut pair = CommensalPair::new(benefactor, neutral, 1.5);
        let gain = pair.commensalize();
        assert_eq!(gain, 1.5);
        assert_eq!(pair.benefactor.fitness, 1.5);
        assert_eq!(pair.neutral.fitness, 5.0); // unchanged
    }

    #[test]
    fn test_detector_basic() {
        let detector = SymbiosisDetector::new(0.5, 10);
        let agents = vec![
            Agent::with_traits("a", vec![1.0, 1.0]),
            Agent::with_traits("b", vec![1.0, 1.0]),
            Agent::with_traits("c", vec![-1.0, -1.0]),
        ];
        let results = detector.detect(&agents);
        // a-b are mutual (high score), a-c and b-c are parasitic (negative)
        assert_eq!(results.len(), 1); // only a-b above threshold
        assert!(results[0].score > 0.9);
    }

    #[test]
    fn test_detector_best_partner() {
        let detector = SymbiosisDetector::new(0.3, 10);
        let agent = Agent::with_traits("x", vec![1.0, 1.0]);
        let candidates = vec![
            Agent::with_traits("a", vec![1.0, 1.0]),
            Agent::with_traits("b", vec![0.0, 0.0]),
            Agent::with_traits("c", vec![-1.0, -1.0]),
        ];
        let (idx, compat) = detector.best_partner(&agent, &candidates).unwrap();
        assert_eq!(idx, 0); // "a" is most compatible
        assert!(compat.score > 0.9);
    }

    #[test]
    fn test_detector_no_matches() {
        let detector = SymbiosisDetector::new(0.99, 10);
        let agents = vec![
            Agent::with_traits("a", vec![1.0, -1.0]),
            Agent::with_traits("b", vec![-1.0, 1.0]),
        ];
        let results = detector.detect(&agents);
        assert!(results.is_empty());
    }

    #[test]
    fn test_evolver_evolve_pair() {
        let a = Agent::new("a", vec![1.0, 0.0], 0.0);
        let b = Agent::new("b", vec![0.0, 1.0], 0.0);
        let mut pair = SymbiontPair::new(a, b);
        let original_a0 = pair.agent_a.traits[0];
        let evolver = SymbiosisEvolver::new(0.1, 10);
        evolver.evolve_pair(&mut pair);
        // After cross-influence, traits should have shifted
        let changed = (pair.agent_a.traits[0] - original_a0).abs() > 0.001;
        assert!(changed);
    }

    #[test]
    fn test_evolver_evolve_population() {
        let mut agents = vec![
            Agent::with_traits("a", vec![1.0]),
            Agent::with_traits("b", vec![1.0]),
            Agent::with_traits("c", vec![-1.0]),
            Agent::with_traits("d", vec![-1.0]),
        ];
        let evolver = SymbiosisEvolver::new(0.1, 1);
        evolver.evolve_population(&mut agents);
        // All should have updated fitness
        for agent in &agents {
            assert!(agent.fitness != 0.0 || agent.traits.iter().all(|&t| t == 0.0));
        }
    }

    #[test]
    fn test_relationship_log() {
        let mut log = RelationshipLog::new();
        log.log("a", "b", SymbiosisType::Mutualism, 1.0, 1);
        log.log("a", "c", SymbiosisType::Parasitism, -0.5, 2);
        assert_eq!(log.len(), 2);
        let a_events = log.events_for("a");
        assert_eq!(a_events.len(), 2);
    }

    #[test]
    fn test_relationship_log_count_by_type() {
        let mut log = RelationshipLog::new();
        log.log("a", "b", SymbiosisType::Mutualism, 1.0, 1);
        log.log("c", "d", SymbiosisType::Mutualism, 2.0, 2);
        log.log("e", "f", SymbiosisType::Parasitism, -1.0, 3);
        let counts = log.count_by_type();
        assert_eq!(*counts.get(&SymbiosisType::Mutualism).unwrap(), 2);
        assert_eq!(*counts.get(&SymbiosisType::Parasitism).unwrap(), 1);
    }
}
