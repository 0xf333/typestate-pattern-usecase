## Implementation Comparison

### With Typestate
- State transitions enforced at compile-time.
- Invalid operations prevented by the type system.
- Clear function availability based on the current state.
- Zero runtime overhead.

### Without Typestate
- State tracked through `Option` fields.
- Runtime checks required.
- Possible to call methods in the wrong order.
- Can lead to runtime errors.

## State Flow

### Unconnected -> Connected -> DataFetched

---

## Usage

1. Clone the repository.
2. Create a `.env` file with your Alchemy API key:
   ```
   ALCHEMY_API_KEY=your_key_here
   ```
3. Run the server:
   ```bash
   cargo run
   ```
4. Open `http://localhost:3000` in your browser.

### Check the terminal for server logs while interacting with the UI.

## Tech Stack
- Rust
- Axum (Web Server)
- Ethers-rs (Ethereum interaction)

<br>