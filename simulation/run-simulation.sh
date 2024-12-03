#!/bin/bash

CONFIG_PARAMETERS_FILE="functions.json"
CONFIG_MUTATIONS_FILE="mutations.json"
RESULTS_DIR="results"
RUNS=30

if ! command -v jq &> /dev/null; then
  echo "jq is required but not installed."
  exit 1
fi

mkdir -p "$RESULTS_DIR"

# For each mutation (4)
for MUTATION in $(jq -r '.mutations[]' "$CONFIG_MUTATIONS_FILE"); do
  SANITIZED_MUTATION="${MUTATION//\//}"
  # For each function (10)
  for FUNC_NAME in $(jq -r 'keys[]' "$CONFIG_PARAMETERS_FILE"); do
    BOUNDS=$(jq -r ".\"$FUNC_NAME\".bounds" "$CONFIG_PARAMETERS_FILE")
    OUTPUT_FILE="$RESULTS_DIR/${FUNC_NAME}_${SANITIZED_MUTATION}.csv"
    # For runs (30)
    for RUN in $(seq 1 $RUNS); do
      echo "Running $FUNC_NAME (Run $RUN/$RUNS) with mutation $MUTATION"
      
       # Execute the command and capture output
      OUTPUT=$(just --justfile ../justfile run-release "$MUTATION" "$FUNC_NAME" "$BOUNDS")
      # Extract headers and results
      HEADER=$(echo "$OUTPUT" | head -n 1)
      RESULT=$(echo "$OUTPUT" | tail -n 1)
      
      if [[ $RUN -eq 1 ]]; then
        echo "$HEADER" > "$OUTPUT_FILE"
        echo "$RESULT" >> "$OUTPUT_FILE"
      else
        echo "$RESULT" >> "$OUTPUT_FILE"
      fi
    done
  done
done