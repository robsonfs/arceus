# Arceus - A dedicated Reverse Proxy Written in Rust

Arceus is a [Rust](https://www.rust-lang.org/) implementation of a dedicated [Reverse Proxy](https://en.wikipedia.org/wiki/Reverse_proxy)

[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

---
**Not intended for production use**
Firstly and foremost, that's a learning project. There is no guarantee that it could be used in production. After being a "Reverse proxy user" for a while, I decided to give a step forward and understand their internal pieces deeply. I'm convinced that the best way to understand something deeply is to build that thing, and that's why I started this project.

If you're looking for a production-ready reverse proxy, I do recommend not reinventing the wheel and picking up a battle-tested option. There are loads of them out there, such as [Nginx](https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/), [Caddy](https://caddyserver.com/docs/quick-starts/reverse-proxy), [Traefik](https://doc.traefik.io/traefik/getting-started/quick-start/), [and so on](https://github.com/dariubs/awesome-proxy). If you're a Rust lover and want something written in Rust, Why not go ahead and give a try to the [Sōzu](https://github.com/sozu-proxy/sozu) project?

---
# Features
- [ ] Client
- [ ] Server
- [ ] Caching
- [ ] Hot reloading configuration
- [ ] ...


# License
The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
