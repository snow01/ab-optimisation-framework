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

---

## About

AB Experimentation and Feature Rollout optimisation framework.

## Features

- Highly performant, with sub microsecond latency, and easy configuration based AB experimentation framework.
- AB Server is built with Rust and takes minimal CPU and memory.
- Experiment configuration supports script based evaluation of context to select eligible experiment for a specific
  cohort of users.
- Experiment configuration supports experiment size specification (either percentage or absolute), where sub cohort of
  specified size is picked for `Test` group, while the remaining cohort is marked for `Control` group. Instrumentation
  tracking data automatically tracks both the `Test` and `Control` groups for an experiment.
- In addition to selecting experiment for specific users, experiments can be frequency constraint (such as only once,
  once every 2 days, etc), where AB framework manages the necessary frequency state.
- Experiments configuration can store configuration metadata to choose application behaviour as per the provided data,
  rather than if-else logic.
- AB framework sends experiment tracking data for instrumentation. Instrumentation is left to the client of the
  framework.
- [Coming soon] Realtime tracking of experimentation performance based on events instrumentation in Kafka or Kinesis.

## Experiment Configuration

- AB Experimentation configurations are defined for an App, known by a name and a short_name.
- Each app has one or more projects, known by a name and a short_name.
- Each project has one or many experiments, each having a name, target audience, and stored data. Target audience is
  either an expression on given `context` or reference to predefined `list`.
- Each target audience has size, specified either as absolute value or percentage.
- Each experiment has one or many variations, each having variation size and a stored data that can be used for
  configuring test behaviour in the application.
- Feature is a specific type of experiment with no variations.
- [TODO] Each project can have one or many experiment groups, which defines a mutual exclusion policy between
  experiments, and priority of experiments. An experiment can be part of only one experiment group.
- Target audiences can also be a predefined list of users for the project - such as beta.

***

- `id`: identifier of the application
- `name`: descriptive name of the application
- `short_name`: short name that is used in tracking data and instrumentation. Should be kept to max 5 characters.
- `projects`:
    - `id`: identifier of the project
    - `name`: descriptive name of the project
    - `short_name`: short name that is used in tracking data and instrumentation. Should be kept to max 5 characters.
    - `experiments`: a project can have multiple experiments, where an experiment is defined by
        - `id`: identifier of the experiment
        - `name`: descriptive name of the experiment
        - `short_name`: short name that is used in tracking data and instrumentation. Should be kept to max 5
          characters.
        - `kind`: experiment are of 2 kinds - `Experiment` or `Feature`, where `Feature` is an `Experiment` without any
          variations.
        - `version`: version number is automatically incremented on updates to the same experiment.
        - `audiences`: experiment is evaluated for audiences, where an `audience` is defined by
            - `name`: descriptive name of the audience
            - `script_src`: optional [**inline expression**](#inline-script-for-experiment-user-selection), which is
              evaluated against provided context to find eligible user cohort. If no script is provided, all users are
              eligible.
            - `audience_list`: optional reference to predefined lists of users, for which this experiment is valid.
            - `size_value`: defines how large subset of users would be picked for experiment `Test` group, remaining set
              of users would be marked for `Control` group. When size is percent value, it can only be from 1 to 100
            - `size_kind`: size can be specified in 2 kinds - `Percent` or `Absolute`, both are self explanatory.
        - `frequency_constraint`: optional [**inline expression**](#inline-script-for-frequency-constraint), which is
          evaluated against experiment tracking data and context to constraint frequency of experiment selection. We can
          specify any arbitrary conditions over the experiment tracking data, many helper functions are provided to ease
          frequency checks: `allow_only_once`, `allow_max_x_times`, `allow_every_x_times`, `allow_once_per_x_period`
          , `allow_once_per_x_period`. If no frequency constraint is provided, users are always selected if they fall in
          the selection cohort.
        - `data`: optional experiment configuration data, which is sent back to client for the active experiment.
        - `variations`: an experiment can have multiple variations, where a variation is randomly picked and does not
          provide any selection expression. A variation is defined by
            - `name`: descriptive name of the variation
            - `short_name`: short name that is used in tracking data and instrumentation. Should be kept to max 5
              characters.
            - `size`: variation size can be defined only in percent terms, and all variation size should add upto
              exactly 100.
            - `data`: optional data that is sent back to client for the active experiment and selected variant. Variant
              data is deep merged with experiment data, where variant data takes priority for same keys.
    - `audience_lists`:
        - `id`: identifier of the audience list
        - `name`: descriptive name of the audience list, eg beta users
        - `list`: list of users

***

## Inline Script for Experiment User Selection

- inline script is a valid [python expression](https://www.w3schools.com/python/python_operators.asp).
- Run context is made available as python dictionary inside `ctx` variable. So, if context
  was `{"new_user": true, "app_version": "4.7.3"}`, we can access new_user value as `ctx['new_user']` and app_version
  value as `ctx['app_version']`. It is advisable to use get function with default value to avoid python key check error,
  eg `ctx.get('app_version', '0.0.0')`.
- All valid python inbuilt functions and operators are supported.
- String literals shall be quoted with single quotes.
- For app version comparison, app version can be parsed into semantic version with method `fns.parse_version`,
  eg `fns.parse_version(ctx['app_version']) > fns.parse_version('4.7.3')` is a valid app version comparison.
- Many helper functions are provided to operate on app version checks:
    - `app_version(ctx)`: returns app_version value from ctx or returns '0.0.0'
    - `lt_version(ctx, version)`: checks if ctx app_version is less than given version.
    - `le_version(ctx, version)`: checks if ctx app_version is less than equal to given version.
    - `gt_version(ctx, version)`: checks if ctx app_version is greater than given version.
    - `ge_version(ctx, version)`: checks if ctx app_version is greater than equal to given version.
    - `eq_version(ctx, version)`: checks if ctx app_version is equal to given version.
    - `ne_version(ctx, version)`: checks if ctx app_version is not equal to given version.
- For any datetime, date, or timedelta operations, following pythong libraries are also provided:
    - `datetime.date`
    - `datetime.time`
    - `datetime.datetime`
    - `datetime.timedelta`

## Inline Script for Frequency Constraint

- Like user selection condition, inline script for frequency constraint is a
  valid [python expression](https://www.w3schools.com/python/python_operators.asp).
- Both the run context and experiment tracking data is made available as python dictionaries: `ctx` and `experiment`
  variables respectively.
- All functions supported for user selection are also supported for frequency constraint. In addition many helper
  functions are provided to ease writing frequency constraints:
    - `allow_only_once(experiment)`: allows experiment to be enabled only once in lifetime.
    - `allow_max_x_times(experiment, times)`: allows experiment to be enabled max times in lifetime.
    - `allow_every_x_times(experiment, times)`: allows experiment to be enabled every x times of its invocation, eg
      every 3 invocation enable it.
    - `allow_once_per_x_period(experiment, weeks=0, days=0, hours=0, minutes=0, seconds=0)`: allows experiment to be
      time distanced from last selection time.

## Experiment tracking data

- Each active experiment is tracked for user with following data:
    - `short_name`: short name of the experiment
    - `selected_version`: version of the experiment, when this experiment was last selected
    - `selected_member_kind`: whether user was part of Test group or Control: where they are represented by T and C,
      respectively.
    - `selected_variation`: if any variation was selected
    - `selection_date`: last selection date of the experiment
    - `total_selection_count`: total number of times this experiment has been selected
    - `invocation_version`: version of the experiment it was invoked last time
    - `invocation_date`: date on which this experiment was invoked last
    - `total_invocation_count`: total times this experiment was invoked.
- All the above values are separated by `|` (pipe) symbol, eg: `rlnch|3|T||2156719|1|3|15|2`
- Selection date is encoded as number of seconds since 1st July 2021.
- Invocation date is encoded as number of seconds since last selection date.
- Multiple experiments data is separated by `~` (tilda) symbol.

## APIs

### Run an experiment

> Finds all the active experiments for a given `app_id`, `project_id`, `user_id`, and `context`.
>
> If project is configured to track experiment via body, client shall save tracking_data sent in response and provide it in next request.
> This tracking data is mandatory to manage instrumentation and frequency constraint, consistently.
>
> If the project is configured to be tracked via cookie, client does not need to do anything extra. Note: cookie size may get bigger if many experiments are configured.
> 
> <b>Selection process:</b>
>> - All active experiments are evaluated.
>> - For each experiment, all audience source are evaluated in an order till user is picked for the experiment.
>>      - For each audience `script_src` is evaluated against provided context data to find if user is eligible for the experiment. If there is no script_src configured, then user is selected by default.
>>      - if user is eligible for the experiment, then user is picked for `Test` group based on configured size_value. 
>>      - Pick logic follows consistent bucketing, where murmur3 hash is calculated for user id and value is normalised between 0 and 10000.
>>      - If size_value is 20, then if user_bucket is < 20 * 100, user is selected for `Test` group, otherwise it is selected for `Control` group.
>> - Frequency constraint is evaluated to check if user is eligible for frequency based display. If no frequency constraint is configured, user is selected by default.
>> - If user was not eligible for experiment, it is not tracked. Though if user was every eligible and experiment is still active, user may still be tracked with old data.
>> - If user was eligible but not passing frequency constraint, then only invocation state of experiment is updated in the tracking data.
>> - If user was eligible and passes frequency constraint too, then all data including selection and invocation state is updated in the tracking data. 

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
  },
  "tracking_data": "rlnch|3|T||2156719|1|3|15|2~crnt|2|T||2156734|2|2|0|2"
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
  ],
  "tracking_data": "rlnch|3|T||2156719|1|3|15|2~crnt|2|T||2156734|2|2|0|2"
}
```

* Tracking Cookie =
    - Cookie Value
      = ```X-abof-j-a=onb,0,T,2; HttpOnly; Path=/; Max-Age=630720000; Expires=Fri, 12 Jul 2041 16:32:42 GMT```
    - Where, cookie name is `X-abof-<app-short-name>-<project-short-name>`
    - **Note:** Clients shall read and use tracking cookie value for event instrumentations.
    
### [App CRUD APIs](docs/api_docs/app_apis.md)

- Add an app
- Update an app data
- View an app
- Get list of apps

### [Project CRUD APIs](docs/api_docs/project_apis.md)

- Add a project for the app
- Update a project data
- View a project data
- Get list of projects for the app

### [Experiment CRUD APIs](docs/api_docs/experiment_apis.md)

- Add an experiment for the app and the project
- Update an experiment data
- View an experiment data
- Get list of experiments for the app and the project

### [Server status APIs](docs/api_docs/server_status.md)

- Change rotation status of the service
- View rotation state of the service

## Getting Started

### Built With

- Service -- Rust v1.53.0
- Admin UI -- streamlit

### Prerequisites

- Rust v1.53.0

### Installation

#### Service

```shell
cd service
cargo build --release
./target/release/ab-optimisation-framework start --config_dir=config --env=dev
```

#### Admin UI

> TODO

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