#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "Usage: source toggle_env.sh"
    return 1 2>/dev/null || exit 1
fi

# Change this to the path to your virtual environment
cd ..
CURRENT_DIR=$(pwd)
VENV_PATH="$CURRENT_DIR/env"

# Check if a virtual environment is currently active
if [[ -n "$VIRTUAL_ENV" ]]; then
    echo "Deactivating virtual environment: $VIRTUAL_ENV"
    deactivate
else
    echo "Activating virtual environment: $VENV_PATH"
    # shellcheck disable=SC1090
    source "$VENV_PATH/bin/activate"
fi
cd scripts

