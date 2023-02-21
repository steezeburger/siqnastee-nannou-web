# siqnastee

### dev workflow
* `npm start` - browser
* `cargo run` - native app

### shitty deploy script
* `npm run build`
* `mv docs/CNAME . && rm -rf docs && mkdir docs && cp dist/* docs/ && mv CNAME docs/`
