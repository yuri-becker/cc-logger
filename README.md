[![AGPL-3.0 License](https://img.shields.io/github/license/yuri-becker/cc-logger?style=for-the-badge&logo=gnu&logoColor=white&color=%23A42E2B )](https://github.com/yuri-becker/cc-logger/blob/latest/LICENSE.md)
[![Docker](https://img.shields.io/docker/pulls/yuribecker/cc-logger?style=for-the-badge&logo=docker&logoColor=white&color=%232496ED
)](https://hub.docker.com/r/yuribecker/cc-logger)
[![Latest Commit](https://img.shields.io/github/last-commit/yuri-becker/cc-logger?style=for-the-badge)](https://github.com/yuri-becker/cc-logger/commits/latest)

<br />
<div align="center">

  <h1 align="center"><strong>CC Logger</strong></h1>

  <p align="center">
    Automates starting and stopping time tracking.
  </p>
</div>
<br/>

## About

This is a browser automation script to start and stop time tracking at Comcave College.

Obviously, this script is only meant to save you a few clicks and not to be run via a cronjob or similar.

## Running

### Docker (recommended)

```sh
docker run --env COMCAVE_USERNAME=username --env COMCAVE_PASSWORD=password yuribecker/cc-logger:latest cc-logger [come|leave|info]
```

### From Source

```sh
COMCAVE_USERNAME=username COMCAVE_PASSWORD=password cargo run [come|leave|info]
```

You can also set environment variables by creating a `.env.local` file when running from source.

### Environment Variables

| Name                       | Description                                                                                               | Default                                   |
|----------------------------|-----------------------------------------------------------------------------------------------------------|-------------------------------------------|
| COMCAVE_USERNAME           | Your username                                                                                             | *required*                                |
| COMCAVE_PASSWORD           | Your password                                                                                             | *required*                                |
| COMCAVE_RANDOM_SLEEP_RANGE | Maximum amount of sleep time in minutes before running the automation                                     | `0`, so no sleep                          |
| COMCAVE_CHROMEDRIVER       | Chromedriver executable                                                                                   | `chromedriver`                            |
| COMCAVE_URL                | Base URL of the Comcave portal                                                                            | `https://portal.cc-student.com/index.php` |
| COMCAVE_NO_SANDBOX         | Sets the `--no-sandbox` flag of Chromium                                                                  | `false`, `true` in Docker                 |
| COMCAVE_HEADLESS           | Sets the `--headless` flag of Chromium                                                                    | `true`                                    |
| RUST_LOG                   | Logging Level, can be `trace`, `debug`, `info`, `warn` and `error`. `info` is recommended for production. | `debug`, `info` in Docker                 |