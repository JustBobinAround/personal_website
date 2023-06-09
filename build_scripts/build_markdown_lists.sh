#!/bin/bash

# Directory path
directory="./assets/blogposts"

# Markdown file path
markdown_file="./assets/bloglist.md"

# Title for the markdown file
title="Blog"

# Create or overwrite the markdown file with the title
echo "# $title" > "$markdown_file"

# Iterate through each file in the directory
for file in "$directory"/*; do
    # Extract the file name
    filename=$(basename "$file")
    
    # Create the new file path for the link
    new_file_path="#/blogposts/$filename"

    # Replace underscores with spaces in the file name
    filename=$(echo "$filename" | tr '_' ' ')
    # Create a clickable link for the file
    link="[${filename%.*}]($new_file_path)"
    
    # Append the link to the markdown file
    echo "- $link" >> "$markdown_file"
done

echo "File list saved in $markdown_file"

