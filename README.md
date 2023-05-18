# Zero To Production with a Twist

Following the Zero to Production book for learning backend development in Rust.

There are some twist to this project, I am self hosting it with the following steps:

- Build a container and store with the GitHub container registery after every push
- Log into the VPS and stop running container
- Pull latest container from the GitHub container registery
- Run the new docker container

NB: A postgres database is created in the VPS instance.

Future plan:

- Automate the deployment to the vps whne a push is made to the main branch in github