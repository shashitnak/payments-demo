# Dodo Payments Backend

## Steps to run the project

1. Clone the project

```commandline
git clone git@github.com:shashitnak/dodopayments.git
```
2. Install docker, if you don't have it already by going [here](https://docs.docker.com/engine/install/)
3. Run the following command to build the docker image
```commandline
docker build . -t dodopayments
```
4. Start the docker container
```commandline
docker run -p 8080:8080 dodopayments
```