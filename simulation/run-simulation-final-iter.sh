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

# Total number of iterations (mutations * functions * runs)
TOTAL_MUTATIONS=$(jq -r '.mutations | length' "$CONFIG_MUTATIONS_FILE")
TOTAL_FUNCTIONS=$(jq -r 'keys | length' "$CONFIG_PARAMETERS_FILE")
TOTAL_RUNS=$((TOTAL_MUTATIONS * TOTAL_FUNCTIONS * RUNS))
CURRENT_RUN=0

# For each mutation
for MUTATION in $(jq -r '.mutations[]' "$CONFIG_MUTATIONS_FILE"); do
  SANITIZED_MUTATION="${MUTATION//\//}"
  # For each function
  for FUNC_NAME in $(jq -r 'keys[]' "$CONFIG_PARAMETERS_FILE"); do
    BOUNDS=$(jq -r ".\"$FUNC_NAME\".bounds" "$CONFIG_PARAMETERS_FILE")
    OUTPUT_FILE="$RESULTS_DIR/${FUNC_NAME}_${SANITIZED_MUTATION}.csv"
    # For runs
    for RUN in $(seq 1 $RUNS); do
      ((CURRENT_RUN++))
      PERCENTAGE=$((CURRENT_RUN * 100 / TOTAL_RUNS))

      echo -ne "\r\033[KRunning $FUNC_NAME (Run $RUN/$RUNS) with mutation $MUTATION | Progress: [$CURRENT_RUN/$TOTAL_RUNS] ${PERCENTAGE}%"

      OUTPUT=$(just --justfile ../justfile run-release "$MUTATION" "$FUNC_NAME" "$BOUNDS")

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