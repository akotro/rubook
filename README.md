# Rubook

`rubook` is a command line application written in [Rust](https://www.rust-lang.org/) that allows users to search for books, manage their collection, and download them as ebooks through Libgen.
This repository contains both the front-end CLI and a backend web API built with [Actix Web](https://actix.rs/).


## Features

- **Search for books**: Easily search for books.
- **Manage your collection**: Keep track of the books you own and the ones you want to read.
- **Download ebooks**: Download books in ebook form to read on your favorite device.

## Installation

1. Go to the [**Releases**](https://github.com/akotro/rubook/releases) page of the repository.
2. Find the latest release and download the `.zip` file for your corresponding platform.
3. Unzip the downloaded file to a location of your choice.
4. Follow the instructions below to modify the .env.sample file with your Google API key.

### Using the Google Books API

This project uses the Google Books API to search for and retrieve information about books. In order to use this API, you will need to obtain a Google API key and add it to a `.env` file in the root of the project.

#### Obtaining a Google API Key

1. Go to the [Google Cloud Console](https://console.cloud.google.com/).
2. Click the project drop-down and select or create the project for which you want to add an API key.
3. Click the hamburger menu and select **APIs & Services > Library**.
4. Search for "Google Books API" and click on the result.
5. Click the **Enable** button to enable the Google Books API for your project.
6. Click the hamburger menu again and select **APIs & Services > Credentials**.
7. On the **Credentials** page, click **Create credentials > API key**.
8. The **API key created** dialog displays your newly created API key. Click **Close**.
9. The new API key is listed on the **Credentials** page under **API keys**.

#### Adding the Google API Key to the Project

1. Create a new file called `.env` in the root of the project.
2. Open the `.env` file in a text editor.
3. Add the following line to the file, replacing `YOUR_GOOGLE_API_KEY` with your Google API key: `GOOGLE_API_KEY=YOUR_GOOGLE_API_KEY`
4. Save and close the `.env` file.

You can also look at or modify the `.env.sample` file provided in the root of the project for an example of how to set up your `.env` file.

## Usage

To use Rubook, simply run the `rubook` command.

## Contributing

Contributions to `rubook` are welcome! If you would like to contribute, please fork the repository and submit a pull request with your changes.

## License

`rubook` is licensed under the MIT license. See [LICENSE](LICENSE) for more information.
