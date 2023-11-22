This repository contains a test for caddy to see how a failed download is handled.

It will stream the file called `testfile.zip` (downloaded from https://www.thinkbroadband.com/download)
and abort the download after a bit.

Run the server (install rust first if needed: https://rustup.rs/) and caddy:
```bash
cargo r
sudo caddy run --config ./Caddyfile
```

Or alternatively, use the very rudimentary dockerfile:
```bash
docker run -p 3030:3030 -p 3031:3031 -p 3032:3032 $(docker build -q .) 
```

Add `caddy.local` to your hostfile pointing to your local machine.

Now try to access `https://caddy.local:3030`, `http://caddy.local:3031` and `https://caddy.local:3032` in your browser.
You'll see the download file for the first two, for the third one (`:3032`), it will seem like the file downloaded succesfully, but it was only
partially downloaded.


