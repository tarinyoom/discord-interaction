echo BUILDING LAMBDA FUNCTION
echo
cargo lambda build --release

echo DEPLOYING LAMBDA FUNCTION
echo
cargo lambda deploy --binary-name demo --iam-role $SOMMELIER_LAMBDA_EXECUTION_ROLE discord_interaction_lambda

echo UPLOADING COMMANDS CONTAINED IN commands.json
echo
curl -X POST -H "Content-Type: application/json" https://discord.com/api/v10/applications/${DEMO_APPLICATION_ID}/commands -H "Authorization: Bot ${DEMO_BOT_TOKEN}" -d @commands.json
