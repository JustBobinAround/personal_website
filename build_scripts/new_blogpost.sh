#!/bin/bash

# Set the working directory
working_directory="./assets/blogposts"

# Get the current date
date=$(date +%Y-%m-%d)

# Prompt the user to enter the article title
read -p "Enter the article title: " title

# Replace spaces in the title with hyphens
file_name=$(echo "$title" | tr '[:space:]' '_')

# Remove any trailing dashes
file_name=$(echo "$file_name" | sed 's/_$//')

# Generate the markdown file name with date and title
markdown_file="(${date})_${file_name}.md"

# Set the working directory
cd "$working_directory" || exit

# Create the markdown file with the title and date
echo "# $title" > "$markdown_file"
echo "Date: $date" >> "$markdown_file"
echo "" >> "$markdown_file"

echo "Template markdown file \"$markdown_file\" created in \"$working_directory\"."

