<h1 align="center">
  <a href="https://github.com/snow01/ab-optimisation-framework">
    <!-- Please provide path to your logo here -->
    <img src="docs/images/logo.png" alt="Logo" height="100">
  </a>
</h1>

<div align="center">
  <b>AB Optimisation Framework</b>
  <br />
  <a href="#about"><strong>Explore the screenshots »</strong></a>
  <br />
  <br />
  <a href="https://github.com/snow01/ab-optimisation-framework/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  ·
  <a href="https://github.com/snow01/ab-optimisation-framework/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
  ·
  <a href="https://github.com/snow01/ab-optimisation-framework/discussions">Ask a Question</a>
</div>

<div align="center">
<br />

[![license](https://img.shields.io/github/license/snow01/ab-optimisation-framework.svg?style=flat-square)](LICENSE)

[![PRs welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/snow01/ab-optimisation-framework/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![code with hearth by snow01](https://img.shields.io/badge/%3C%2F%3E%20with%20%E2%99%A5%20by-snow01-ff1414.svg?style=flat-square)](https://github.com/snow01)

</div>

<details open="open">
<summary>Table of Contents</summary>

- [About](#about)
    - [Built With](#built-with)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
- [Usage](#usage)
    - [APIs](#apis)
      - [Run an experiment](#run-an-experiment)
      - [Add Or Update an app](#add-or-update-an-app)
      - [View an app](#view-an-app)
      - [Add Or Update a project](#add-or-update-a-project)
      - [View a project](#view-a-project)
      - [Change rotation status of service](#change-rotation-status-of-service)
      - [View rotation status of service](#view-rotation-status-of-service)
- [Roadmap](#roadmap)
- [Support](#support)
- [Project assistance](#project-assistance)
- [Contributing](#contributing)
- [Authors & contributors](#authors--contributors)
- [Security](#security)
- [License](#license)
- [Acknowledgements](#acknowledgements)

</details>

---

## About

Multi Arm Bandit based AB Experimentation and Feature Rollout optimisation framework.

<br/>
<details>
<summary>Screenshots</summary>

|                               Home Page                               |                               Login Page                               |
| :-------------------------------------------------------------------: | :--------------------------------------------------------------------: |
| <img src="docs/images/screenshot.png" title="Home Page" width="100%"> | <img src="docs/images/screenshot.png" title="Login Page" width="100%"> |

</details>

### Built With

- Service -- Rust v1.53.0
- Admin UI -- streamlit

## Getting Started

### Prerequisites

- Rust v1.53.0

### Installation

#### Service
```shell
cd service
cargo build
./target/debug/ab-optimisation-framework start --config_dir=config --env=dev
```

#### Admin UI

> TODO

## Usage

### APIs

#### Run an experiment

Finds all the active experiments for a given app, project, user, and context.

* URL = `/api/run`

* Method = `POST`

* Request Body =
```json
{
  "app_id": "app1",
  "project_id": "android",
  "user_id": "123",
  "context": {
    "app_version": "4.7.1004",
    "new_user": true
  }
}
```

* Response Body =
```json
{
  "app_id": "app1",
  "project_id": "android",
  "tracking_cookie_name": "X-abof-j-a",
  "active_experiments": [
    {
      "short_name": "onb",
      "variation": "2",
      "data": {
        "a": 2,
        "b": "b2",
        "c": "c2",
        "d": 0,
        "e": [
          "e0",
          "e1",
          "e2",
          "e20",
          "e21",
          "e22"
        ],
        "f": {
          "f1": "f1",
          "f12": "f12"
        }
      }
    }
  ]
}
```

* Tracking Cookie: ```X-abof-j-a=onb,0,T,2; HttpOnly; Path=/; Max-Age=630720000; Expires=Fri, 12 Jul 2041 16:32:42 GMT```

Note: Clients shall read and use tracking cookie value for event instrumentations.

#### Add or Update an App

* URL = `/api/app`

* Method = `POST`

* Request Body =
```json
{
  "id": "app1",
  "name": "josh",
  "short_name": "j"
}
```

### View an App

* URL = `/api/app/<app-id>`

* Method = `GET`
  
* Response Body =
```json
{
  "id": "app1",
  "name": "josh",
  "short_name": "j"
}
```

### Add or Update a Project

* URL = `/api/project`

* Method = `POST`
  
* Request Body =
```json
{
  "id": "android",
  "name": "android",
  "short_name": "a",
  "app": "app1",
  "experiments": [
    {
      "id": "onboarding",
      "name": "onboarding",
      "short_name": "onb",
      "version": 0,
      "kind": "Experiment",
      "audiences": [
        {
          "name": "new_users",
          "size_kind": "Percent",
          "size_value": 20,
          "audience_kind": "Script",
          "script_src": "ctx['new_user'] == True and ctx['app_version'] >= '4.7.3'"
        },
        {
          "name": "beta_users",
          "size_kind": "Absolute",
          "size_value": 2,
          "audience_kind": "List",
          "list_id": "beta_users"
        }
      ],
      "variations": [
        {
          "id": "variation1",
          "name": "variation 1",
          "short_name": "1",
          "size": 20,
          "data": {
            "a": 1,
            "b": "b1",
            "c": "c1",
            "e": [
              "e10",
              "e11",
              "e12"
            ],
            "f": {
              "f11": "f11"
            }
          }
        },
        {
          "id": "variation2",
          "name": "variation 2",
          "short_name": "2",
          "size": 80,
          "data": {
            "a": 2,
            "b": "b2",
            "c": "c2",
            "e": [
              "e20",
              "e21",
              "e22"
            ],
            "f": {
              "f12": "f12"
            }
          }
        }
      ],
      "data": {
        "a": 0,
        "b": "b0",
        "c": "c0",
        "d": 0,
        "e": [
          "e0",
          "e1",
          "e2"
        ],
        "f": {
          "f1": "f1"
        }
      }
    }
  ],
  "audience_lists": [
    {
      "id": "beta_users",
      "name": "beta users",
      "list": [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9"
      ]
    }
  ]
}
```

#### View a Project

* URL = `/api/project/<app-id>/<project-id>`

* Method = `GET`
  
* Response body =
```json
{
  "id": "android",
  "name": "android",
  "short_name": "a",
  "app": "app1",
  "experiments": [
    {
      "id": "onboarding",
      "name": "onboarding",
      "short_name": "onb",
      "version": 0,
      "kind": "Experiment",
      "audiences": [
        {
          "name": "new_users",
          "size_kind": "Percent",
          "size_value": 20,
          "audience_kind": "Script",
          "script_src": "ctx['new_user'] == True and ctx['app_version'] >= '4.7.3'"
        },
        {
          "name": "beta_users",
          "size_kind": "Absolute",
          "size_value": 2,
          "audience_kind": "List",
          "list_id": "beta_users"
        }
      ],
      "variations": [
        {
          "id": "variation1",
          "name": "variation 1",
          "short_name": "1",
          "size": 20,
          "data": {
            "a": 1,
            "b": "b1",
            "c": "c1",
            "e": [
              "e10",
              "e11",
              "e12"
            ],
            "f": {
              "f11": "f11"
            }
          }
        },
        {
          "id": "variation2",
          "name": "variation 2",
          "short_name": "2",
          "size": 80,
          "data": {
            "a": 2,
            "b": "b2",
            "c": "c2",
            "e": [
              "e20",
              "e21",
              "e22"
            ],
            "f": {
              "f12": "f12"
            }
          }
        }
      ],
      "data": {
        "a": 0,
        "b": "b0",
        "c": "c0",
        "d": 0,
        "e": [
          "e0",
          "e1",
          "e2"
        ],
        "f": {
          "f1": "f1"
        }
      }
    }
  ],
  "audience_lists": [
    {
      "id": "beta_users",
      "name": "beta users",
      "list": [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9"
      ]
    }
  ]
}
```
 
#### Change rotation status of service

Inverts the OOR status of the service and returns new status

* URL = `/oor`

* Method = `GET`

* Response = `one of the string: OK, NOK`

#### View rotation status of service

Returns the rotation status of the service

* URL = `/status`

* Method = `GET`

* Response = `one of the string: OK, NOK`

## Design Details

- AB Experimentation configurations are defined for an App, known by a name.
- Each app has one or more projects, known by a name.
- Each project has one or many experiments, each having a name, target audience, and stored data. Target audience is either an expression on given `context` or reference to predefined `list`.
- Each target audience has size, specified either as absolute value or percentage.
- Each experiment has one or many variations, each having variation size and a stored data that can be used for configuring test behaviour in the application.
- Feature is a specific type of experiment with no variations.
- Each project can have one or many experiment groups, which defines a mutual exclusion policy between experiments, and priority of experiments. An experiment can be part of only one experiment group.
- Target audiences can also be a predefined list of users for the environment - such as beta.


## Roadmap

- SDKs for 
  - Java Server
  - iOS App
  - Android App
  - Web App

See the [open issues](https://github.com/snow01/ab-optimisation-framework/issues) for a list of proposed features (and
known issues).

- [Top Feature Requests](https://github.com/snow01/ab-optimisation-framework/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc) (
  Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/snow01/ab-optimisation-framework/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc) (
  Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/snow01/ab-optimisation-framework/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

## Support

Reach out to the maintainer at one of the following places:

- [GitHub discussions](https://github.com/snow01/ab-optimisation-framework/discussions)
- [Shailendra Sharma](https://github.com/snow01)

## Project assistance

If you want to say **thank you** or/and support active development of AB Optimisation Framework:

- Add a [GitHub Star](https://github.com/snow01/ab-optimisation-framework) to the project.
- Tweet about the AB Optimisation Framework on your Twitter.
- Write interesting articles about the project on [Dev.to](https://dev.to/), [Medium](https://medium.com/) or personal
  blog.

Together, we can make AB Optimisation Framework **better**!

## Contributing

First off, thanks for taking the time to contribute! Contributions are what make the open-source community such an
amazing place to learn, inspire, and create. Any contributions you make will benefit everybody else and are **greatly
appreciated**.

We have set up a separate document containing our [contribution guidelines](docs/CONTRIBUTING.md).

Thank you for being involved!

## Authors & contributors

The original setup of this repository is by [Shailendra Sharma](https://github.com/snow01).

For a full list of all authors and contributors,
check [the contributor's page](https://github.com/snow01/ab-optimisation-framework/contributors).

## Security

AB Optimisation Framework follows good practices of security, but 100% security can't be granted in software. AB
Optimisation Framework is provided **"as is"** without any **warranty**. Use at your own risk.

_For more info, please refer to the [security](docs/SECURITY.md)._

## License

This project is licensed under the **MIT license**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

* Bapu Kota, for his reviews and suggestions.