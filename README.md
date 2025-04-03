<!-- @format -->

a# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

### Tailwind

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

npm install tailwindcss @tailwindcss/cli

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

```

target/release/assets

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform desktop
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform web
```

## Inconify Icons

Material Symbols Light by Google

dx build --release

## Bundling

### Mac OS

dx bundle --platform desktop \
 --package-types "macos" \
 --package-types "dmg"

[dependencies]
