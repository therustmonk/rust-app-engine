# Rust App Engine

This projects is a minimal boilerplate ro run Rust web application inside Google App Engine.

To deploy it use Google Cloud Shell:

    ```sh
    $ gcloud preview app deploy
    ```

> ***Note:*** Sources builds automatically inside docker container. But if you build it locally,
> GCS takes time to upload `target` content which never used. Remove `target` before deployment.
