# Roadmap
1. Create basic structs for user and account info
   - [x] Create user struct
   - [x] Create account struct
   - [x] Create transaction struct
   - [x] Create budget struct
   - [ ] Create category struct
2. Add functions for income and spending for each account
3. Build JSON "database"
4. Build TUI
   - [ ] User input handling
   - [ ] Multiple windows/screens
   -- GOTO: https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
5. Use Plaid API to connect to bank account for live information
   - [ ] Create Plaid account & get API keys
   - [ ] Connect to Plaid API
   - [ ] Connect to bank account
   - [ ] Pull live information
   - [ ] Store information in database (either local Pocketbase server or Firebase)
   -- GOTO: https://plaid.com/docs/quickstart/
    + Plaid documentation
   -- GOTO: https://docs.rs/plaid/latest/plaid/model/index.html
    + Rust API for Plaid 
6. Establish/migrate to NoSQL database
      - [ ] Create database
7. Build GUI
   -- GOTO: https://crates.io/crates/druid
      + Rust GUI library
      + `cargo add druid`
      + or add 'druid = "0.8.3"' to Cargo.toml
   - [ ] Try to match Figma design
   - [ ] Connect GUI to database
   - [ ] Display information
