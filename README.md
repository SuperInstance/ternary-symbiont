# ternary-symbiont: Symbiotic relationships between ternary agents

## Why This Exists

When ternary agents share a room, they don't just coexist — they develop relationships. Some help each other (mutualism), some exploit others (parasitism), and some benefit without affecting their neighbor (commensalism). These patterns emerge from trait compatibility and evolve over time. This crate models all three relationship types, detects beneficial partnerships automatically, and co-evolves pairs to strengthen (or weaken) their bonds.

## Core Concepts

- **Agent**: An entity with a trait vector (ternary-valued: -1.0, 0.0, +1.0) and a fitness score. Traits determine compatibility with other agents.
- **SymbiosisType**: `Mutualism` (both benefit), `Parasitism` (one gains, one loses), `Commensalism` (one gains, one unaffected). Determined by cosine similarity of trait vectors.
- **Compatibility**: Computed as normalized dot product (cosine similarity) between two agents' trait vectors. Score ranges from -1.0 to +1.0. Above 0.3 = mutualism, below -0.3 = parasitism, between = commensalism.
- **SymbiontPair**: Mutualistic pair. Each `interact()` call gives both agents a fitness boost proportional to compatibility × bond strength. Bond strength grows with each interaction.
- **ParasiticPair**: One parasite, one host. `parasitize()` drains fitness from host at a configurable rate. Tracks how many cycles until the host is depleted.
- **CommensalPair**: One benefactor, one neutral agent. `commensalize()` gives the benefactor a fitness gain without affecting the neutral agent.
- **SymbiosisDetector**: Scans a pool of agents and returns the best partnerships sorted by compatibility score. Can find the single best partner for a given agent.
- **SymbiosisEvolver**: Co-evolves pairs by cross-influencing traits — each agent's traits shift slightly toward its partner's. Also supports population-level evolution.
- **RelationshipLog**: Records relationship events with agent IDs, type, fitness delta, and tick number. Supports querying by agent and counting by type.

## Quick Start

```toml
[dependencies]
ternary-symbiont = "0.1"
```

```rust
use ternary_symbiont::*;

// Create agents with ternary traits
let agent_a = Agent::with_traits("scout", vec![1.0, 0.0, 1.0]);
let agent_b = Agent::with_traits("analyst", vec![1.0, 0.0, 1.0]);

// Check compatibility
let compat = compute_compatibility(&agent_a, &agent_b);
assert_eq!(compat.relationship, SymbiosisType::Mutualism);

// Form a symbiotic pair and interact
let mut pair = SymbiontPair::new(agent_a, agent_b);
pair.interact();
pair.interact();
assert!(pair.combined_fitness() > 0.0);
assert!(pair.bond_strength > 1.0);

// Detect partnerships in a pool
let detector = SymbiosisDetector::new(0.5, 5);
let pool = vec![
    Agent::with_traits("a", vec![1.0, 1.0]),
    Agent::with_traits("b", vec![1.0, 1.0]),
    Agent::with_traits("c", vec![-1.0, -1.0]),
];
let partnerships = detector.detect(&pool);
```

## API Overview

| Type | Description |
|------|-------------|
| `Agent` | Agent with id, trait vector, and fitness score |
| `TernaryValue` | Not in this crate — traits use f64: -1.0, 0.0, +1.0 |
| `SymbiosisType` | Enum: `Mutualism`, `Parasitism`, `Commensalism` |
| `Compatibility` | Score + relationship type from trait comparison |
| `compute_compatibility()` | Free function: cosine similarity of trait vectors |
| `SymbiontPair` | Mutualistic pair with bond strength and interaction |
| `ParasiticPair` | Parasite + host with drain rate and survival tracking |
| `CommensalPair` | Benefactor + neutral agent with benefit rate |
| `SymbiosisDetector` | Scans agent pools for best partnerships |
| `SymbiosisEvolver` | Co-evolves pairs and populations over generations |
| `RelationshipLog` | Event log with agent queries and type counting |

## How It Works

Compatibility is cosine similarity: the dot product of two trait vectors divided by the product of their norms. This gives a score in [-1, 1]. The relationship type is determined by thresholding: >0.3 mutualism, <-0.3 parasitism, else commensalism. This maps directly to biological symbiosis classification.

SymbiontPair interactions use a feedback loop: higher compatibility → more fitness gain → stronger bond → even more gain next cycle. This models real mutualistic relationships that strengthen over time. ParasiticPair is simpler: fixed drain rate per cycle until the host runs out. CommensalPair is the simplest: fixed benefit to one party with no feedback.

The evolver uses trait cross-influence: for each trait dimension, agents shift toward their partner's value by `mutation_rate × 0.1 × difference`. This is gradient ascent on compatibility. A small deterministic perturbation is added based on fitness and trait index (to avoid needing an RNG).

## Known Limitations

- No random number generation. Mutations are deterministic based on fitness and trait index, which limits evolutionary diversity.
- Cosine similarity with ternary-valued traits produces a limited set of possible scores. With all -1/0/+1 traits of length N, there are only O(N²) distinct compatibility scores.
- The evolver's cross-influence is symmetrical, which drives traits toward convergence. Without mutation pressure, all agents in a population eventually become identical.
- No support for multi-agent relationships (3+ agents in a single symbiosis). Only pairwise.
- RelationshipLog grows without bound. No pruning or summarization.

## Use Cases

1. **Room cohabitation dynamics**: Two agents assigned to the same Codespace room develop a mutualistic relationship over repeated interactions, increasing both their fitness and effectiveness at shared tasks.
2. **Adversarial resource competition**: A parasitic agent drains a host's compute budget. The host must detect this and relocate or sever the relationship before it's depleted.
3. **Auto-discovery of team compositions**: SymbiosisDetector scans all agents in a fleet to find the best natural partnerships, suggesting room assignments that maximize mutual fitness.

## Ecosystem Context

Works with `ternary-agent` (individual agents that develop relationships), `ternary-fitness` (fitness scoring used here), and `ternary-swarm` (which places agents into rooms where symbioses can form). The relationship log feeds into `ternary-evolution` for long-term evolutionary analysis. Related to `ternary-game-theory` for competitive/cooperative strategy analysis.

## License

MIT

## See Also
- **ternary-evolution-advanced** — related
- **ternary-ga** — related
- **ternary-fitness** — related
- **ternary-genome** — related
- **ternary-swarm** — related

