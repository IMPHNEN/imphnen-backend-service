#!/bin/bash

set_temp_env_from_dotenv() {
    local env_file_path="$1"

    if [[ ! -f "$env_file_path" ]]; then
        echo "Error: The .env file at path '$env_file_path' does not exist."
        return 1
    fi

    while IFS= read -r line || [[ -n "$line" ]]; do
        trimmed_line=$(echo "$line" | xargs)

        if [[ -n "$trimmed_line" && ! "$trimmed_line" =~ ^# ]]; then
            key=$(echo "$trimmed_line" | cut -d '=' -f 1 | xargs)
            value=$(echo "$trimmed_line" | cut -d '=' -f 2- | xargs)
            export "$key=$value"
            echo "Set temporary environment variable: $key=$value"
        fi
    done < "$env_file_path"

    echo "All environment variables from '$env_file_path' have been set temporarily."
}

set_temp_env_from_dotenv ".env"
