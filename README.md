# Rust CTL

A flexible api server that defines object structure from a yaml file, and maps it to routes and a MySQL DB.  

# TL;DR: this is a work in progres, just started

## The objects.yaml file

Modify objects.yaml to define yours:  
```
object_type: host  <- This will become the object, and also a table on your MySQL DB.  
attributes:        <- Each of these make a column on that table.  
  - title: hostname  <- Name of the column and the attribute at the API.  
    mysql_scheme: "char(60) DEFAULT ''"  <- MySQL scheme for the table.  
    command: test  <- Command used to get the current value of the attribute at the machine level.  
```


