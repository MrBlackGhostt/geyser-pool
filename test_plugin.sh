#!/bin/bash

# 1. Define paths
CONFIG_FILE="$(pwd)/config.json"
LEDGER_DIR="$(pwd)/test-ledger"
LOG_FILE="$(pwd)/account_update_logs.txt"
VALIDATOR_LOG="$(pwd)/validator.log"

# 2. Cleanup previous runs
echo "Cleaning up previous run artifacts..."
rm -rf "$LEDGER_DIR"
rm -f "$LOG_FILE"
rm -f "$VALIDATOR_LOG"

# 3. Start the validator in the background
echo "Starting solana-test-validator with the plugin..."
# Redirect both stdout and stderr to validator.log
solana-test-validator \
	--ledger "$LEDGER_DIR" \
	--geyser-plugin-config "$CONFIG_FILE" \
	--rpc-port 8899 \
	--faucet-port 9900 \
	>"$VALIDATOR_LOG" 2>&1 &
VALIDATOR_PID=$!

echo "Validator PID: $VALIDATOR_PID"

# 4. Wait for validator to start
echo "Waiting 15 seconds for validator to initialize..."
sleep 15

# Check if validator is still running
if ! kill -0 $VALIDATOR_PID 2>/dev/null; then
	echo "CRITICAL: Validator process died unexpectedly!"
	echo "Tail of validator log:"
	tail -n 20 "$VALIDATOR_LOG"
	exit 1
fi

# 5. Trigger an account update (airdrop) to generate logs
echo "Triggering an account update via airdrop..."
# We use a loop to retry if it fails initially
for i in {1..5}; do
	if solana airdrop 1 --url http://127.0.0.1:8899; then
		echo "Airdrop successful!"
		break
	else
		echo "Airdrop failed (attempt $i). Retrying in 2s..."
		sleep 2
	fi
done

# 6. Wait a moment for the plugin to write to disk
sleep 2

# 7. Check if the log file exists and print content
echo "Checking for log file at: $LOG_FILE"
if [ -f "$LOG_FILE" ]; then
	echo "SUCCESS: Log file found!"
	echo "---------------------------------------------------"
	head -n 5 "$LOG_FILE"
	echo "..."
	echo "---------------------------------------------------"
else
	echo "FAILURE: Log file was not created."
	echo "Checking validator log for plugin errors:"
	grep -i "plugin" "$VALIDATOR_LOG" | tail -n 10
fi

# 8. Cleanup: Stop the validator
echo "Stopping validator..."
kill $VALIDATOR_PID
wait $VALIDATOR_PID 2>/dev/null || true
echo "Validator stopped."
