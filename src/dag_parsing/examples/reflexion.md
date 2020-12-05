* either TOML or YAML (or HJSON)
    * parser produce an intermediate form
    * intermediate form is instanciated into a DAG
* so a config file need to define four things:
    * nodes
    * definitions
    * runner
    * dag
* it should be possible to declare nodes(definition)+dag
    or nodes+definitions+dag or dag(node(definition))
    linking via ids
