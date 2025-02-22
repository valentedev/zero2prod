To start DB config and initial migration execute ./scripts/init_db.sh

To format test log output
TEST_LOG=true cargo test | bunyan  
Require 'cargo install bunyan'

=== Deploy preparation ===

Docker build --tag zero2prod .

'sqlx prepare' performs the same work of cargo build but same the outcome in a .sqlx file. Dockerfile needs to receive the instruction: ENV SQLX_OFFLINE true 
cargo sqlx prepare --workspace -- --all-targets


Deploy to Digital Ocean using spec.yaml
doctl apps create --spec spec.yaml

Disable trusted sources on Digital Ocean

Migrate database
DATABASE_URL=postgresql://newsletter:AVNS_ckWLV9WtY2h3WA1LusL@app-95f8fb4e-43ff-42f3-bb1b-16584537e494-do-user-4836224-0.k.db.ondigitalocean.com:25060/newsletter?sslmode=require sqlx migrate run

curl --request POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' 127.0.0.1:8000/subscriptions --verbose 

Update deploy...
doctl apps update <APP-ID-NUMBER> --spec=spec.yaml


