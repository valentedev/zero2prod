To start DB config and initial migration execute ./scripts.init_db.sh

To format test log output
TEST_LOG=true cargo test | bunyan  
Require 'cargo install bunyan'

=== Deploy preparation ===

Docker build --tag zero2prod .

'sqlx prepare' performs the same work of cargo build but same the outcome in a .sqlx file. Dockerfile needs to receive the instruction: ENV SQLX_OFFLINE true 
cargo sqlx prepare --workspace -- --all-targets

curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' 127.0.0.1:8000/subscriptions --verbose   

