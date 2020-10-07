/*
When we mount the Dag,
    it pops up a list of nodes to execute
    then while that list is not empty
    it takes on node and runs it
        when a node is ran
        it checks if it's complete via the output/complete
        if yes, skip and maybe return task_instance
        if not, it runs the task_definition
        when a node is done,
        we add its children to the list
*/

#[test]
fn can_mount_simple_dag() {}
