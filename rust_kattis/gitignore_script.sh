#!/bin/bash

# Store the current working directory
original_dir=$(pwd)

# Find all directories in the current directory
for dir in */ ; do
    # Change into the directory
    cd "$original_dir/$dir"
    
    # Check if ".git" directory exists, indicating it's a git repository
    if [ -d ".git" ]; then
        echo "Processing $dir"
        
        # Run the git command
        git rm -r --cached target/
        
        # Optional: commit the change
        # git commit -m "Remove target/ from cache"
    else
        echo "$dir is not a git repository, skipping..."
    fi
done

# Change back to the original directory
cd "$original_dir"

