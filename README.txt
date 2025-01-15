To start DB config and initial migration execute ./scripts.init_db.sh

To format test log output
TEST_LOG=true cargo test | bunyan  
Require 'cargo install bunyan'

=== Deploy preparation ===

Docker build --tag zero2prod .

'sqlx prepare' performs the same work of cargo build but same the outcome in a .sqlx file 
cargo sqlx prepare --workspace -- --all-targets

