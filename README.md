# Yoshi

// todo: add badges here :)

*Implementing something like Python's Luigi but with Rust*

*Build complex pipelines of batch jobs*

![logo-with-yoshi](https://seeklogo.com/images/Y/yoshi-logo-15F601923A-seeklogo.com.png)

## Getting started

## Background

## Contributing

## Technologies

## Todos
(aka tech debt aka improvements)

[Trello board](https://trello.com/b/UugKGxA1/yoshi-rs)

* TaskDefinition
  * add trait method forcing human readable name
  * add supports for success&failure callbacks
  * add a task that runs queries in Postgres (any db)
  * PythonTaskDefinition
    * add tests in case of virtualenv
    * correctly capture stdout
* TaskOutput
  * rework the logic behind complete/output
* TaskRunner
* Write a complete E2E test like
  * create a Dag
  * add a Node A that does a small bash command
  * add a Node B with parent A that does a small python command
  * base Node B completion on some file created by Node A
* DagBuilder
* DagRunner
* Others
  * add test coverage
  * reconsider what library to use for executable
  * move datetime functions to own module
