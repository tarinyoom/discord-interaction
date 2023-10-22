cargo lambda build --release
cargo lambda deploy --binary-name demo --iam-role $SOMMELIER_LAMBDA_EXECUTION_ROLE discord_interaction_lambda
