# zRest

Rust library for the zulip rest API

## Usefull links

- [Dev server](https://zulip.readthedocs.io/en/latest/development/setup-vagrant.html)
- [Rest API](https://zulip.com/api/rest)


## To set up tests

```shell
git clone https://github.com/zulip/zulip         
cd zulip              
vagrant up --provider=virtualbox  # Yes it must be vbox :(                     
vagrant ssh           
./tools/run-dev.py
# Get http://localhost:9991/devlogin/ to check it works
^C # Controll-C, to close
exit
vagrant halt
vagrant snapshot save test_clean_start
```

## To run tests

```shell
vagrant up
vagrant ssh         
./tools/run-dev.py
# In another tab
cargo test
```

## To clean up tests

```shell
^C # shut down dev server
exit
vagrant halt    
vagrant snapshot restore test_clean_start      
```

## Launching the dev server (outdated)

```shell
systemctl start docker
sudo setenforce 0
vagrant up --provider=docker
vagrant ssh 
./tools/run-dev.py
```