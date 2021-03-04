# zRest

Rust library for the zulip rest API

## Usefull links

- [Dev server](https://zulip.readthedocs.io/en/latest/development/setup-vagrant.html)
- [Rest API](https://zulip.com/api/rest)

## Launching the dev server

```shell
systemctl start docker
sudo setenforce 0
vagrant up --provider=docker
vagrant ssh 
./tools/run-dev.py
```

## Running tests

```shell
git clone https://github.com/zulip/docker-zulip/
cd docker-zulip
sd /opt/docker/zulip/ `pwd`/data_run/ docker-compose.yml
docker-compose pull
docker-compose build # Takes like an hour
docker-compose up # Takes a load of time
```

```shell
git clone https://github.com/zulip/zulip
vagrant up --provider=docker
vagrant ssh       
./tools/run-dev.py
# Get http://localhost:9991/devlogin/ to check it works
^C # Controll-C, to close
exit
vagrant halt
vagrant snapshot save test_clean_start
```

```shell
git clone https://github.com/zulip/zulip
vagrant up --provider=virtualbox
vagrant ssh       
./tools/run-dev.py

```

## To set up tests

```shell
git clone https://github.com/zulip/zulip         
cd zulip              
vagrant up --provider=virtualbox                        
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
```