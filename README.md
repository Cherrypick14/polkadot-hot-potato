# 🥔 Hot Potato Game - Polkadot Smart Contract

A decentralized, time-based multiplayer game built on Polkadot using ink! smart contracts. Players must pass the "hot potato" to each other before time runs out, or risk being eliminated!

## 🎯 Project Overview

The Hot Potato Game is a blockchain-based implementation of the classic children's game, where players must quickly pass a potato to avoid being the last one holding it when time expires. Built with [ink!](https://use.ink/) (Rust-based smart contract framework for Polkadot), this project demonstrates:

- **Smart Contract Development** with ink! v6.0.0-alpha
- **Time-based Game Mechanics** using blockchain blocks
- **Multi-player Interaction** on decentralized networks
- **Event-driven Architecture** (currently commented out, ready for implementation)
- **Comprehensive Testing** including unit tests and end-to-end tests

## 🏗️ Architecture

### Smart Contract Structure

The contract is built using ink! and consists of:

- **Core Game Logic**: Start, pass, and end game functionality
- **State Management**: Track current holder, game status, and timing
- **Access Control**: Only current holder can pass the potato
- **Time Management**: Block-based deadline system
- **Query Functions**: Read-only access to game state

### Key Components

```rust
pub struct Hotpotato {
    current_holder: Option<AccountId>,      // Who currently holds the potato
    last_passed_block: u32,                 // Block when potato was last passed
    deadline_blocks: u32,                   // Blocks until deadline
    active: bool,                           // Game status
    game_starter: Option<AccountId>,        // Who initiated the game
}
```

## 🎮 Game Mechanics

### How to Play

1. **Start Game**: Any player can start a new game by calling `start_game(to: AccountId)`
   - Sets the initial potato holder
   - Activates the game
   - Records the game starter

2. **Pass Potato**: Current holder must call `pass_potato(to: AccountId)` before time expires
   - Transfers potato to another player
   - Updates the last passed block
   - Resets the deadline timer

3. **Check Deadline**: Anyone can call `check_deadline()` to verify if time has expired
   - Automatically ends game if deadline passed
   - Last holder gets eliminated
   - Game resets to inactive state

4. **End Game**: Game starter can manually end the game using `end_game()`
   - Resets all game state
   - Useful for testing or early termination

### Time System

- **Deadline**: Configurable block count from contract deployment
- **Timer**: Resets each time the potato is passed
- **Elimination**: Last holder when deadline expires loses

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [cargo-contract](https://github.com/paritytech/cargo-contract) for ink! development
- [Substrate node](https://github.com/substrate-developer-hub/substrate-node-template) with `pallet-contracts` for testing

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Cherrypick14/polkadot-hot-potato
   cd polkadot-hot-potato
   ```

2. **Install cargo-contract**
   ```bash
   cargo install --locked --git https://github.com/use-ink/cargo-contract
   ```

3. **Build the contract**
   ```bash
   cd hotpotato
   cargo contract build
   ```

### Running Tests

1. **Unit Tests**
   ```bash
   cargo test
   ```

2. **End-to-End Tests**
   ```bash
   cargo test --features e2e-tests
   ```

## 📋 Contract Functions

### Core Functions

| Function | Description | Access |
|----------|-------------|---------|
| `new(deadline_blocks: u32)` | Constructor - Initialize game with deadline | Public |
| `start_game(to: AccountId)` | Start new game with initial holder | Public |
| `pass_potato(to: AccountId)` | Pass potato to another player | Current holder only |
| `check_deadline()` | Check if deadline expired and end game | Public |
| `end_game()` | Manually end game | Game starter only |

### Query Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `get_holder()` | Get current potato holder | `Option<AccountId>` |
| `is_active()` | Check if game is active | `bool` |
| `get_deadline_blocks()` | Get deadline block count | `u32` |
| `get_remaining_blocks()` | Get blocks until deadline | `u32` |
| `get_game_starter()` | Get who started the game | `Option<AccountId>` |

## 🧪 Testing

### Unit Tests

The contract includes comprehensive unit tests covering:

- ✅ Contract initialization
- ✅ Game start functionality
- ✅ Potato passing mechanics
- ✅ Deadline checking
- ✅ Game ending
- ✅ Error conditions and edge cases

### End-to-End Tests

E2E tests verify contract behavior in a real Substrate environment:

- ✅ Contract instantiation
- ✅ Game flow integration
- ✅ Event emission (when implemented)
- ✅ Cross-account interactions

## 🔮 Future Enhancements

### Planned Features

- **Event System**: Implement commented-out events for better frontend integration
- **Frontend DApp**: Web interface for game interaction
- **Token Integration**: Add rewards/penalties using native tokens
- **Multi-Game Support**: Allow multiple concurrent games
- **Advanced Time Mechanics**: More sophisticated deadline systems
- **Player Statistics**: Track wins, losses, and participation

### Event System (Ready for Implementation)

```rust
// Currently commented out, ready to implement:
// self.env().emit_event(GameAction {
//     action_type: 1, // start
//     player: caller,
// });
```

## 🛠️ Development

### Project Structure

```
polkadot-hot-potato/
├── hotpotato/           # Main smart contract
│   ├── Cargo.toml      # Dependencies and build config
│   ├── lib.rs          # Contract implementation
│   └── target/         # Build artifacts
├── ink-node            # Substrate node for testing
├── LICENSE             # MIT License
└── README.md           # This file
```

### Dependencies

- **ink**: Smart contract framework for Polkadot
- **scale**: Parity's SCALE codec for efficient serialization
- **scale-info**: Type information for ink! contracts
- **ink_e2e**: End-to-end testing framework

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust coding standards
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass before submitting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Parity Technologies](https://www.parity.io/) for ink! framework
- [Polkadot](https://polkadot.network/) ecosystem
- [Substrate](https://substrate.io/) for blockchain infrastructure

##  Support

- **Issues**: Report bugs and request features via GitHub Issues
- **Discussions**: Join community discussions for questions and ideas
- **Documentation**: Check ink! docs at [use.ink](https://use.ink/)

---

**Built with ❤️ using ink! and Polkadot**

*The Hot Potato Game demonstrates the power of decentralized applications and smart contract development on modern blockchain networks.* 