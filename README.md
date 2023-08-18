# Zero To Production with a Twist

![CI](https://github.com/morukele/zero-to-prod/actions/workflows/general.yml/badge.svg)

Following the Zero to Production book for learning backend development in Rust.

## Twists

There are some twist to this project, I am self hosting it with the following steps:

- Build a container and store with the GitHub container registery after every push
- Log into the VPS and stop running container
- Pull latest container from the GitHub container registery
- Run the new docker container

NB: A postgres database is created in the VPS instance.s

Running on a VPS instead of an app platform leads me to implement things like autoscaling myself via HAproxy.

For the email client, Sendgrid is used as a provider and the Rust unofficial sendgird client by [gsquire](https://github.com/gsquire/sendgrid-rs) is used in the code base.

## Deployment

The deployment of the application is done using docker-compose.

When a new version of the app is created, a container is built and stored in the docker container registry.

The docker compose file is then updated to reflect the new version and restarted.
