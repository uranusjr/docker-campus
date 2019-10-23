# Manage all docker-compose projects on your machine in one command

## Usage

### Listing

```
$ docker-campus list
Name        Root
-----------------------
dearbnb     C:\Users\uranusjr\Documents\programming\dearbnb
tej-jet     C:\Users\uranusjr\Documents\programming\tej-jet
```

`docker-campus add <name> <root>` adds a new project.

`docker-campus remove <name>` removes a project.


### Management

```
$ docker-campus compose <name> <command> [<arg> ...]
```

Run `docker-compose <command> <arg> ...` on the project specified by `<name>`.
The implementation is to simply `chdir` to the project’s root directory and
call `docker-compose`.
