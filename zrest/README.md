# zRest

Rust library for the zulip rest API

## Usefull links

- [Dev server](https://zulip.readthedocs.io/en/latest/development/setup-vagrant.html)
- [Rest API](https://zulip.com/api/rest)

## Launching the dev server

```
systemctl start docker
sudo setenforce 0
vagrant up --provider=docker
vagrant ssh 
./tools/run-dev.py
```