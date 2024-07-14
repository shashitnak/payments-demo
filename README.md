# Dodo Payments Backend

## Steps to run the project

1. Clone the project

```commandline
git clone git@github.com:shashitnak/dodopayments.git
```
2. Install docker, if you don't have it already, by going [here](https://docs.docker.com/engine/install/)
3. Run the following command to build the docker image.
```commandline
docker build . -t dodopayments
```
4. Start the docker container.
```commandline
docker run -p 8080:8080 dodopayments
```
5. The server is now running at `http://127.0.0.1:8080`. See the [documentation](https://documenter.getpostman.com/view/36976530/2sA3kPoPjd#9c8ad3a6-6603-4a69-88e5-b4e1a24f5c66).