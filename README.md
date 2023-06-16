# Personal Website

Welcome to my personal website repository! This project serves as my personal website hosted on GitHub Pages. It is written in Rust using the Yew front-end framework. The website utilizes a custom markdown component that asynchronously fetches markdown files from a remote source using the `reqwest` library. The fetched markdown files are then converted to HTML using the `pulldown-cmark` library and rendered for users to view.

**Disclaimer: This website is currently under development and may not always have content available.**

## Technologies Used

- Rust: The programming language used to write the website.
- Yew: A modern Rust framework for creating web applications.
- Reqwest: A Rust HTTP client library used for making asynchronous requests to fetch markdown files.
- pulldown-cmark: A Rust library for parsing and rendering CommonMark Markdown.
- Bash: Simple bash scripts are included to automate the creation of markdown files, such as blog posts.

## Features

- Fetch and Render Markdown: The website includes a custom markdown component that fetches remote markdown files asynchronously. The fetched markdown is then converted to HTML using the `pulldown-cmark` library and rendered on the website.
- Automated Markdown Creation: Simple bash scripts are provided to automate the creation of markdown files, particularly for tasks like blog posts.


## Usage

To use this project and set up your own personal website:

1. Clone the repository: `git clone https://github.com/JustBobinAround/personal_website.git`
2. Customize the content and design of the website according to your preferences.
3. Create markdown files using the provided bash scripts or manually.
4. Run the website locally for testing and development.
5. Deploy the website to GitHub Pages or any other hosting platform of your choice. There is also a deployment script in the make file. Read the wiki for more details


### Running Locally with Trunk

This project utilizes Trunk as a build tool for running the website locally. To run the site locally, follow these steps:

1. Install Trunk if you haven't already: `cargo install trunk`
2. Make sure you have `make` installed on your system.
3. In the project directory, run the following command: `make run`
4. Trunk will build and serve the website locally.
5. Open your web browser and visit `http://localhost:8080` to view the website.

## Contributions

Contributions to this personal website project are welcome! If you find any issues, have suggestions for improvements, or want to add new features, feel free to open an issue or submit a pull request.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
